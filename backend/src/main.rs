#[macro_use]
extern crate diesel;

use crate::authentication::reject_anonymous_users;
use actix_cors::Cors;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use actix_web_lab::web::spa;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use secrecy::{ExposeSecret, Secret};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod authentication;
mod handlers;
mod models;
mod schema;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let index_file = "./dist/index.html".to_string();

    let hostname = std::env::var("SERVER_FQDN").expect("HOSTNAME not set");
    let port = std::env::var("PORT").expect("PORT not set");
    let disable_https: bool = std::env::var("DISABLE_HTTPS")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap();
    let url = format!(
        "{}://{}:{}",
        if disable_https { "http" } else { "https" },
        hostname,
        port
    );

    //Inject environment variables used by frontend into index.html
    let mut index = std::fs::read_to_string(&index_file).unwrap();
    index = index.replace("__SERVER_FQDN__", &hostname);
    index = index.replace("__PORT__", &port);
    index = index.replace("__DISABLE_HTTPS__", &disable_https.to_string());
    std::fs::write(&index_file, index).unwrap();

    let redis_uri: Secret<String> = Secret::new(
        std::env::var("REDIS_URL")
            .expect("REDIS_URL not set")
            .to_string(),
    );
    let hmac_secret: Secret<String> = Secret::new(
        std::env::var("REDIS_PASSWORD")
            .expect("Password invalid")
            .to_string(),
    );
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret())
        .await
        .unwrap();
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(url.as_str())
            .allow_any_method()
            .supports_credentials()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .service(handlers::auth::login)
                    .route("", web::get().to(|| async { "Actix REST API" }))
                    .service(handlers::users::whoami)
                    .service(handlers::tickets::options)
                    .service(handlers::tickets::index)
                    .service(handlers::tickets::by_assignee)
                    .service(handlers::tickets::create)
                    .service(handlers::tickets::show)
                    .service(handlers::tickets::update)
                    .service(handlers::tickets::destroy)
                    .service(handlers::tickets::revisions)
                    .service(handlers::tickets::events)
                    .service(handlers::users::index)
                    .service(handlers::users::create)
                    .service(handlers::users::show)
                    .service(handlers::users::update)
                    .service(handlers::users::destroy)
                    .service(handlers::user_preferences::get_preferences)
                    .service(handlers::user_preferences::update_preferences)
                    .service(handlers::contacts::index)
                    .service(handlers::contacts::create)
                    .service(handlers::contacts::show)
                    .service(handlers::contacts::update)
                    .service(handlers::contacts::delete)
                    .service(handlers::notes::index)
                    .service(handlers::notes::create)
                    .service(handlers::notes::show)
                    .service(handlers::notes::update)
                    .service(handlers::notes::delete)
                    .service(handlers::notes::ticket_notes)
                    .service(handlers::auth::logout)
                    .service(handlers::documents::index)
                    .service(handlers::documents::create)
                    .service(handlers::documents::show)
                    .service(handlers::documents::update)
                    .service(handlers::documents::delete)
                    .service(handlers::documents::revisions)
                    .wrap(from_fn(reject_anonymous_users)),
            )
            .service(
                spa()
                    .index_file(index_file.clone())
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
    });
    if disable_https {
        server.bind(("0.0.0.0", port.parse().unwrap()))?.run().await
    } else {
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth();

        let cert_file = &mut std::io::BufReader::new(
            std::fs::File::open("certificates/cert.pem")
                .unwrap_or_else(|e| panic!("Error opening certificate file: {}", e)),
        );
        let key_file = &mut std::io::BufReader::new(
            std::fs::File::open("certificates/key.pem")
                .unwrap_or_else(|e| panic!("Error opening certificate file: {}", e)),
        );

        // Parse the certificate and set it in the configuration
        let cert_chain = certs(cert_file)
            .unwrap()
            .into_iter()
            .map(Certificate)
            .collect();
        let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
            .unwrap()
            .into_iter()
            .map(PrivateKey)
            .collect();
        let config = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();

        server
            .bind_rustls(format!("0.0.0.0:{}", port), config)?
            .run()
            .await
    }
}
