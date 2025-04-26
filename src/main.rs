mod config;
mod models;
mod handlers;
mod middleware;


use sqlx::mysql::MySqlPoolOptions;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::io;
use dotenv::dotenv;
use std::env;
use handlers::user_handler::{get_all_user, insert_user, updatedata, delete, search};
use actix_cors::Cors;
use actix_web::http;

//auth bagian
use actix_web_httpauth::middleware::HttpAuthentication;
use handlers::auth::login;
use crate::middleware::jwt_middleware;


async fn status() -> impl Responder {
    HttpResponse::Ok().body("OK Server Berhasil diakses")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok(); // Load from .env

    let db_url = env::var("DATABASE_URL").expect("database url salah atau error");
    let db_pool = MySqlPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("gagal menyambung mysql");

    let config = config::AppConfig::from_env()
        .expect("Gagal memuat konfigurasi dari environment");
    println!("sukses terhubung mysql");

    println!(
        "Server berjalan di http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
     let cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::CONTENT_TYPE])
        .max_age(3600);

    App::new()
        .wrap(cors)
        .app_data(web::Data::new(db_pool.clone()))
        .route("/", web::get().to(status))
        .service(login)
        .service(
            web::scope("/admin")
            .wrap(HttpAuthentication::bearer(jwt_middleware))
            .service(insert_user)
            .service(get_all_user)
            .service(updatedata)
            .service(delete)
            .service(search)
        )
 
    })
    
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
