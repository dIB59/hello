use reqwest::Client;
use crate::models::payment::{PaymentIntentResponse, CreatePaymentRequest, ConfirmPaymentResponse};

pub async fn create_payment(amount: u64, currency: &str) -> Result<PaymentIntentResponse, reqwest::Error> {
    let client = Client::new();
    let api_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    let url = "https://api.stripe.com/v1/payment_intents";

    let payment_intent = CreatePaymentRequest {
        amount,
        currency: currency.to_string(),
        payment_method_types: vec!["card".to_string()],
    };

    let res = client
        .post(url)
        .bearer_auth(api_key)
        .form(&payment_intent)
        .send()
        .await?
        .json::<PaymentIntentResponse>()
        .await?;

    Ok(res)
}

pub async fn confirm_payment(payment_intent_id: &str) -> Result<ConfirmPaymentResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let api_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    let url = format!("https://api.stripe.com/v1/payment_intents/{}/confirm", payment_intent_id);

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
    let url = format!("https://api.stripe.com/v1/payment_intents/{}", payment_id);

    let res = client
        .get(&url)
        .bearer_auth(api_key)
        .send()
        .await?
        .json::<PaymentIntentResponse>()
        .await?;

    Ok(res.status)
}
