use std::env;

use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

//Sub is the email of the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    let header = Header::new(Algorithm::HS256);
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}

mod tests {
    use super::*;

    #[test]
    fn test_create_jwt() {
        dotenv::dotenv().ok();
        let user_id = "123";
        let token = create_jwt(user_id);
        let claims = decode_jwt(&token).unwrap();
        assert_eq!(claims.sub, user_id);

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .expect("valid timestamp")
            .timestamp() as usize;
        assert_eq!(claims.exp, expiration);
    }

    #[test]
    fn test_decode_jwt() {
        dotenv::dotenv().ok();
        let user_id = "123";
        let token = create_jwt(user_id);
        let claims = decode_jwt(&token).unwrap();
        assert_eq!(claims.sub, user_id);

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .expect("valid timestamp")
            .timestamp() as usize;
        assert_eq!(claims.exp, expiration);

        let invalid_token = "invalid_token";
        let result = decode_jwt(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_jwt_expired() {
        dotenv::dotenv().ok();
        let user_id = "123";
        let expiration = chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::days(1))
            .expect("valid timestamp")
            .timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration,
        };
        let header = Header::new(Algorithm::HS256);
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();
        let result = decode_jwt(&token);
        assert!(result.is_err());
    }
}
