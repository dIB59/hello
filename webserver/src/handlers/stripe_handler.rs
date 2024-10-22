use actix_web::{HttpResponse, web, post};
use crate::services::stripe_service;
use crate::models::payment::{CreatePaymentRequest, ConfirmPaymentRequest};
use log::error;

pub async fn create_payment(req: web::Json<CreatePaymentRequest>) -> HttpResponse {
    let CreatePaymentRequest { amount, ref currency, ref payment_method_types }  = req.0; 

    let payment_service = stripe_service::PaymentService::new(stripe_service::get_stripe_base_url());
    match payment_service.create_payment(amount, currency).await {
        Ok(payment_intent) => HttpResponse::Ok().json(payment_intent),
        Err(e) => {
            error!("Error creating payment: {}", e);
            HttpResponse::BadGateway().json("Failed to create payment")
        }
    }
}

pub async fn confirm_payment(req: web::Json<ConfirmPaymentRequest>) -> HttpResponse {
    let ConfirmPaymentRequest { payment_intent_id } = req.0;

    let payment_service = stripe_service::PaymentService::new(stripe_service::get_stripe_base_url());
    match payment_service.confirm_payment(&payment_intent_id).await {
        Ok(confirm_response) => HttpResponse::Ok().json(confirm_response),
        Err(e) => {
            error!("Error confirming payment: {}", e);
            HttpResponse::BadGateway().json("Failed to confirm payment")
        }
    }
}

pub async fn check_payment_status(payment_id: web::Path<String>) -> HttpResponse {
    let payment_id = payment_id.into_inner();

    let payment_service = stripe_service::PaymentService::new(stripe_service::get_stripe_base_url());
    match payment_service.check_payment_status(&payment_id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => {
            error!("Error checking payment status: {}", e);
            HttpResponse::BadGateway().json("Failed to check payment status")
        }
    }
}

pub fn payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .route("/create_payment", web::post().to(create_payment))
            .route("/confirm_payment", web::post().to(confirm_payment))
            .route("/check_status/{payment_id}", web::get().to(check_payment_status))
    );
}
