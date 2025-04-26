use actix_web::{post, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::{Login, Admin};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct WaktuToken {
    sub: String,
    exp: usize,
}

#[post("/login")]
pub async fn login(
    db: web::Data<MySqlPool>,
    kerahasian: web::Json<Login>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Admin,
        "SELECT id, username, password FROM admin WHERE username = ?",
        kerahasian.username
    )
    .fetch_optional(db.get_ref())
    .await;

    match result {
        Ok(Some(admin)) => {
            if admin.password == kerahasian.password {
                let rahasia = env::var("JWT_SECRET").unwrap_or_else(|_| "rahasia".into());

                let claims = WaktuToken {
                    sub: admin.username,
                    exp: (Utc::now().timestamp() + 60 * 60) as usize, // 1 jam
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(rahasia.as_bytes()),
                )
                .unwrap();

                HttpResponse::Ok().json(serde_json::json!({ "token": token }))
            } else {
                HttpResponse::Unauthorized().body("Password salah")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Username tidak ditemukan"),
        Err(e) => {
            println!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Terjadi kesalahan")
        }
    }
}
