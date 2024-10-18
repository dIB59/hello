use crate::models::payment::{PaymentIntentResponse, ConfirmPaymentResponse, CreatePaymentRequest};
use reqwest::Client;
use std::env;

pub struct PaymentService {
    client: Client,
    base_url: String,
}

impl PaymentService {
    pub fn new(base_url: String) -> Self {
        PaymentService {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn create_payment(
        &self,
        amount: u64,
        currency: &str,
    ) -> Result<PaymentIntentResponse, reqwest::Error> {
        let api_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

        let payment_intent = CreatePaymentRequest {
            amount,
            currency: currency.to_string(),
            payment_method_types: vec!["card".to_string()],
        };

        let res = self
            .client
            .post(format!("{}/v1/payment_intents", self.base_url)) 
            .bearer_auth(api_key)
            .json(&payment_intent)
            .send()
            .await?
            .json::<PaymentIntentResponse>()
            .await?;

        Ok(res)
    }

    pub async fn confirm_payment(
        &self,
        payment_intent_id: &str,
    ) -> Result<ConfirmPaymentResponse, reqwest::Error> {
        let api_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

        let res = self
            .client
            .post(format!(
                "{}/v1/payment_intents/{}/confirm",
                self.base_url, payment_intent_id
            ))
            .bearer_auth(api_key)
            .send()
            .await?
            .json::<ConfirmPaymentResponse>()
            .await?;

        Ok(res)
    }

 
    pub async fn check_payment_status(
        &self,
        payment_id: &str,
    ) -> Result<String, reqwest::Error> {
        let api_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

        let res = self
            .client
            .get(format!("{}/v1/payment_intents/{}", self.base_url, payment_id))
            .bearer_auth(api_key)
            .send()
            .await?
            .json::<PaymentIntentResponse>()
            .await?;

        Ok(res.status)
    }
}


pub fn get_stripe_base_url() -> String {
    if cfg!(test) {
        mockito::server_url()  
    } else {
        "https://api.stripe.com".to_string()  
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};
    use actix_rt::test;

    fn get_test_payment_service() -> PaymentService {
        let base_url = mockito::server_url();
        PaymentService::new(base_url)
    }

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

        let payment_service = get_test_payment_service();
        let amount = 1000;
        let currency = "usd";

        let result = payment_service.create_payment(amount, currency).await;
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

        let payment_service = get_test_payment_service();
        let result = payment_service.confirm_payment(payment_intent_id).await;

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

        let payment_service = get_test_payment_service();
        let result = payment_service.check_payment_status(payment_intent_id).await;

        assert!(result.is_ok(), "Payment status check failed when it should have succeeded");

        let status = result.unwrap();
        assert_eq!(status, "requires_action");
    }
}
