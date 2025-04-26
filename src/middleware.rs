use actix_web::{dev::ServiceRequest, Error, error::ErrorUnauthorized};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::handlers::auth::WaktuToken;
use std::env;
use futures_util::future::{ready, Ready};

/// Middleware untuk memvalidasi JWT
pub fn jwt_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Ready<Result<ServiceRequest, (Error, ServiceRequest)>> {
    let token = credentials.token();
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "rahasia".to_string());

    let decoded = decode::<WaktuToken>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    match decoded {
        Ok(_token_data) => ready(Ok(req)),
        Err(err) => {
            println!("Token error: {:?}", err);
            ready(Err((ErrorUnauthorized("Token tidak valid"), req)))
        }
    }
}
