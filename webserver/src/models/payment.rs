use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub amount: u64,  
    pub currency: String,
    pub payment_method_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentIntentResponse {
    pub id: String,
    pub client_secret: String,
    pub status: String,
}

#[derive(Deserialize, Serialize)]
pub struct ConfirmPaymentRequest {
    pub payment_intent_id: String,
}

#[derive(Deserialize, Serialize)] 
pub struct ConfirmPaymentResponse {
    pub id: String,
    pub status: String,
    pub client_secret: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct CheckPaymentStatusResponse {
    pub status: String,
}
