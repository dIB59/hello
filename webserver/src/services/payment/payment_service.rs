use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("Stripe error: {0}")]
    StripeErrorPayment(#[from] stripe::StripeError),
    // BlockchainErrorPayment,
    #[error("Invalid currency")]
    InvalidCurrency,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Invalid InvalidParams")]
    InvalidParams,
}

pub enum PaymentStatus {
    Succeeded,
    Pending,
    RequiresAction(String),
    Failed,
}

#[async_trait]
pub trait PaymentService {
    type PaymentIntent;  // Abstract over PaymentIntent type
    type Status;         // Abstract over Status type

    async fn create_payment_intent(&self, amount: i64, currency: &str) -> Result<Self::PaymentIntent, PaymentError>;
    async fn confirm_payment_intent(&self, payment_intent_id: &str) -> Result<Self::PaymentIntent, PaymentError>;
    async fn check_payment_status(&self, payment_intent_id: &str) -> Result<Self::Status, PaymentError>;
    async fn charge_payment(&self, payment_intent_id: &str) -> Result<Self::PaymentIntent, PaymentError>;
}

pub enum PaymentMethod {
    Stripe,
    Blockchain,
}
