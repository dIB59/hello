use crate::services::payment::payment_service::PaymentError::StripeErrorPayment;
use crate::services::payment::payment_service::{PaymentError, PaymentService, PaymentStatus};
use async_trait::async_trait;
use serde_json::Value;
use std::error::Error;
use std::str::FromStr;
use stripe::{
    Client, CreatePaymentIntent, ErrorType, PaymentIntent, PaymentIntentConfirmParams,
    PaymentIntentId, StripeError,
};

pub struct StripePaymentService {
    client: Client,
}

impl StripePaymentService {
    pub fn new(secret_key: String) -> Self {
        let client = Client::new(secret_key);
        StripePaymentService { client }
    }
}

impl From<stripe::PaymentIntentStatus> for PaymentStatus {
    fn from(status: stripe::PaymentIntentStatus) -> Self {
        match status {
            stripe::PaymentIntentStatus::Succeeded => PaymentStatus::Succeeded,
            stripe::PaymentIntentStatus::RequiresAction => {
                PaymentStatus::RequiresAction(status.to_string())
            }
            stripe::PaymentIntentStatus::Canceled => PaymentStatus::Failed,
            stripe::PaymentIntentStatus::Processing => PaymentStatus::Pending,
            stripe::PaymentIntentStatus::RequiresCapture => {
                PaymentStatus::RequiresAction(status.to_string())
            }
            stripe::PaymentIntentStatus::RequiresConfirmation => {
                PaymentStatus::RequiresAction(status.to_string())
            }
            stripe::PaymentIntentStatus::RequiresPaymentMethod => {
                PaymentStatus::RequiresAction(status.to_string())
            }
        }
    }
}

#[async_trait]
impl PaymentService for StripePaymentService {
    type PaymentIntent = stripe::PaymentIntent;
    type Status = stripe::PaymentIntentStatus;

    async fn create_payment_intent(
        &self,
        amount: i64,
        currency: &str,
    ) -> Result<Self::PaymentIntent, PaymentError> {
        let currency =
            stripe::Currency::from_str(currency).map_err(|_| PaymentError::InvalidCurrency)?;

        let payment_intent = stripe::CreatePaymentIntent::new(amount, currency);

        match stripe::PaymentIntent::create(&self.client, payment_intent).await {
            Ok(intent) => Ok(intent),
            Err(err) => {
                log::error!("Error creating payment intent: {:?}", err);
                Err(PaymentError::from(err))
            }
        }
    }

    async fn confirm_payment_intent(
        &self,
        payment_intent_id: &str,
    ) -> Result<Self::PaymentIntent, PaymentError> {
        let payment_intent_id = payment_intent_id.to_string();
        let payment_params = stripe::PaymentIntentConfirmParams::default();
        let payment_intent =
            stripe::PaymentIntent::confirm(&self.client, &payment_intent_id, payment_params).await;

        match payment_intent {
            Ok(intent) => Ok(intent),
            Err(err) => {
                log::error!("Error confirming payment intent: {:?}", err);
                Err(PaymentError::from(err))
            }
        }
    }

    async fn check_payment_status(
        &self,
        payment_intent_id: &str,
    ) -> Result<Self::Status, PaymentError> {
        let payment_id = stripe::PaymentIntentId::from_str(payment_intent_id)
            .map_err(|_| PaymentError::InvalidParams)?;

        let status = stripe::PaymentIntent::retrieve(&self.client, &payment_id, &[])
            .await
            .map(|intent| intent.status);

        match status {
            Ok(status) => Ok(status),
            Err(err) => {
                log::error!("Error checking payment status: {:?}", err);
                Err(PaymentError::from(err))
            }
        }
    }

    async fn charge_payment(
        &self,
        payment_intent_id: &str,
    ) -> Result<Self::PaymentIntent, PaymentError> {
        let safe_payment_intent_id = PaymentIntentId::from_str(payment_intent_id)
            .map_err(|_| PaymentError::InvalidParams)?;

        let payment_amount = PaymentIntent::retrieve(&self.client, &safe_payment_intent_id, &[])
            .await
            .map(|intent| intent.amount)
            .map_err(|err| PaymentError::from(err))?;

        let capture_params = stripe::CapturePaymentIntent {
            amount_to_capture: Some(payment_amount as u64),
            application_fee_amount: None,
        };
        PaymentIntent::capture(&self.client, &safe_payment_intent_id, capture_params)
            .await
            .map_err(|err| PaymentError::from(err))
    }
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_payment_intent() {
        dotenv::dotenv().ok();
        let stripe_secret =
            std::env::var("STRIPE_TEST_SECRET_KEY").expect("STRIPE_TEST_SECRET_KEY must be set");
        let service = StripePaymentService::new(stripe_secret);
        let intent = service.create_payment_intent(100, "usd").await.unwrap();
        assert_eq!(intent.amount, 100);
        assert_eq!(intent.currency, stripe::Currency::USD);
    }

    #[tokio::test]
    async fn test_confirm_payment_intent() {
        dotenv::dotenv().ok();

        let stripe_secret =
            std::env::var("STRIPE_TEST_SECRET_KEY").expect("STRIPE_TEST_SECRET_KEY must be set");
        let service = StripePaymentService::new(stripe_secret);
        let intent = service.create_payment_intent(100, "usd").await.unwrap();
        let confirmed_intent = service.confirm_payment_intent(&intent.id).await.unwrap();
        assert_eq!(
            confirmed_intent.status,
            stripe::PaymentIntentStatus::RequiresPaymentMethod
        );
    }

    #[tokio::test]
    async fn test_check_payment_status() {
        dotenv::dotenv().ok();

        let stripe_secret =
            std::env::var("STRIPE_TEST_SECRET_KEY").expect("STRIPE_TEST_SECRET_KEY must be set");
        let service = StripePaymentService::new(stripe_secret);
        let intent = service.create_payment_intent(100, "usd").await.unwrap();
        let status = service.check_payment_status(&intent.id).await.unwrap();
        assert_eq!(status, stripe::PaymentIntentStatus::RequiresPaymentMethod);

        let confirmed_intent = service.confirm_payment_intent(&intent.id).await.unwrap();
        let status = service
            .check_payment_status(&confirmed_intent.id)
            .await
            .unwrap();
        assert_eq!(status, stripe::PaymentIntentStatus::Succeeded);
    }

    #[tokio::test]
    async fn test_charge_payment() {
        dotenv::dotenv().ok();

        let stripe_secret =
            std::env::var("STRIPE_TEST_SECRET_KEY").expect("STRIPE_TEST_SECRET_KEY must be set");
        let service = StripePaymentService::new(stripe_secret);
        let intent = service.create_payment_intent(100, "usd").await.unwrap();

        let confirmed_intent = service.confirm_payment_intent(&intent.id).await.unwrap();
        let status = service.charge_payment(&confirmed_intent.id).await.unwrap();
        assert_eq!(status.status, stripe::PaymentIntentStatus::Succeeded);
    }
}
