use serde::{Deserialize, Serialize};
use stripe::Client;

#[derive(Serialize, Deserialize)]
pub struct StripeService {
    pub client: Client,
}

impl StripeService {
    pub fn new() -> Self {
        Self {
            client: Client::n
        }
    }
}