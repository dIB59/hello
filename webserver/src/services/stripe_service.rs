use reqwest::Client;
use crate::models::payment::{PaymentIntentResponse, CreatePaymentRequest, ConfirmPaymentResponse};
use actix_web::web::Path;
    use crate::models::payment::ConfirmPaymentRequest;
    
pub async fn create_payment(amount: u64, currency: &str) -> Result<PaymentIntentResponse, reqwest::Error> {
    let client = Client::new();
    let api_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    let url = if cfg!(test) {
        format!("{}/v1/payment_intents", mockito::server_url())
    } else {
        "https://api.stripe.com/v1/payment_intents".to_string()
    };


    let payment_intent = CreatePaymentRequest {
        amount,
        currency: currency.to_string(),
        payment_method_types: vec!["card".to_string()],
    };


    let res = client
        .post(url)
        .bearer_auth(api_key)
        .json(&payment_intent)
        .send()
        .await?
        .json::<PaymentIntentResponse>()
        .await?;

    Ok(res)
}

pub async fn confirm_payment(payment_intent_id: &str) -> Result<ConfirmPaymentResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let api_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    let url = if cfg!(test) {
        format!("{}/v1/payment_intents/{}/confirm", mockito::server_url(), payment_intent_id)
    } else {
        format!("https://api.stripe.com/v1/payment_intents/{}/confirm", payment_intent_id)
    };

    let res = client
        .post(&url)
        .bearer_auth(api_key)
        .send()
        .await?
        .json::<ConfirmPaymentResponse>()
        .await?;

    Ok(res)
}



pub async fn check_payment_status(payment_id: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let api_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    let url = if cfg!(test) {
        format!("{}/v1/payment_intents/{}", mockito::server_url(), payment_id)
    } else {
        format!("https://api.stripe.com/v1/payment_intents/{}", payment_id)
    };

    let res = client
        .get(&url)
        .bearer_auth(api_key)
        .send()
        .await?
        .json::<PaymentIntentResponse>()
        .await?;

    Ok(res.status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};
    use actix_rt::test;

    #[actix_rt::test]
    async fn test_create_payment_success() {
        let _mock = mock("POST", "/v1/payment_intents")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"
                {
                    "id": "pi_123456789",
                    "client_secret": "secret_123456",
                    "status": "requires_payment_method",
                    "amount": 1000,
                    "currency": "usd"
                }

            "#)
            .create();
        std::env::set_var("STRIPE_SECRET_KEY", "test_secret_key");

        let amount = 1000;
        let currency = "usd";
        let result = create_payment(amount, currency).await;
        assert!(result.is_ok(), "Payment creation failed when it should have succeeded");

        let payment_intent = result.unwrap();

        assert_eq!(payment_intent.id, "pi_123456789");
        assert_eq!(payment_intent.status, "requires_payment_method");
        assert_eq!(payment_intent.amount, amount);
        assert_eq!(payment_intent.currency, currency);
    }

    #[actix_rt::test]
    async fn test_confirm_payment_success() {
        let payment_intent_id = "pi_123456789";
        let _mock = mock("POST", format!("/v1/payment_intents/{}/confirm", payment_intent_id).as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"
                {
                    "id": "pi_123456789",
                    "client_secret": "secret_123456",
                    "status": "succeeded"
                }
            "#)
            .create();

        std::env::set_var("STRIPE_SECRET_KEY", "test_secret_key");
        let result = confirm_payment(payment_intent_id).await;
   
        assert!(result.is_ok(), "Payment confirmation failed when it should have succeeded");

        let confirmed_payment = result.unwrap();
        assert_eq!(confirmed_payment.id, "pi_123456789");
        assert_eq!(confirmed_payment.status, "succeeded");
    }

    #[actix_rt::test]
    async fn test_check_payment_status_success() {
        let payment_intent_id = "pi_123456789";
        let _mock = mock("GET", format!("/v1/payment_intents/{}", payment_intent_id).as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"
                {
                    "id": "pi_123456789",
                    "client_secret": "secret_123456",
                    "status": "requires_action",
                    "amount": 1000,
                    "currency": "usd"
                }
            "#)
            .create();

        std::env::set_var("STRIPE_SECRET_KEY", "test_secret_key");

        let result = check_payment_status(payment_intent_id).await;

        assert!(result.is_ok(), "Payment status check failed when it should have succeeded");

        let status = result.unwrap();
        assert_eq!(status, "requires_action");
    }
}