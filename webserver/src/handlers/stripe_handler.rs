use actix_web::{HttpResponse, web, post, Responder};
use crate::services::stripe_service;
use crate::models::payment::{CreatePaymentRequest, CheckPaymentStatusResponse, PaymentIntentResponse, ConfirmPaymentRequest};
use reqwest::Client;
use serde_json::json;


pub async fn create_payment(req: web::Json<CreatePaymentRequest>) -> HttpResponse {
    let CreatePaymentRequest { amount, ref currency, ref payment_method_types }  = req.0; 
    // Extract data from the request and pass it to the service layer
    match stripe_service::create_payment(req.amount, &req.currency).await {
        Ok(payment_intent) => HttpResponse::Ok().json(payment_intent),
        Err(e) => {
            eprintln!("Error creating payment: {}", e);
            HttpResponse::InternalServerError().json("Failed to create payment")
        }
    }
}

pub async fn confirm_payment(req: web::Json<ConfirmPaymentRequest>) -> HttpResponse {
    let ConfirmPaymentRequest { payment_intent_id } = req.0;

    match stripe_service::confirm_payment(&payment_intent_id).await {
        Ok(confirm_response) => HttpResponse::Ok().json(confirm_response),
        Err(e) => {
            eprintln!("Error confirming payment: {}", e);
            HttpResponse::InternalServerError().json("Failed to confirm payment")
        }
    }
}



pub async fn check_payment_status(payment_id: web::Path<String>) -> HttpResponse {
    let payment_id = payment_id.into_inner();
    match stripe_service::check_payment_status(&payment_id).await {
        Ok(status) => HttpResponse::Ok().json(CheckPaymentStatusResponse { status }),
        Err(e) => {
            eprintln!("Error checking payment status: {}", e);
            HttpResponse::InternalServerError().json("Failed to check payment status")
        }
    }
}





pub fn payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")  
            .route("/create_payment", web::post().to(create_payment)) 
            .route("/confirm_payment", web::post().to(confirm_payment))
            .route("/check_status/{payment_id}", web::get().to(check_payment_status)) 
    );
}
