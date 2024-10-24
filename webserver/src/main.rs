extern crate diesel;

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use actix::Actor;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::{self, Data};
use actix_web::{App, HttpServer};
use dotenv::dotenv;

use chat::chat_server;
use config::Config;
use database::db::MIGRATIONS;

use reqwest::blocking::Client;
use search::data_sync;
use search::state::{self, SearchState};

use crate::database::db;

mod auth;
mod chat;
mod config;
mod database;
mod extractors;
mod handlers;
mod macros_local;
mod models;
mod routes;
mod schema;
mod services;
mod search;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    dotenv().ok();

    let database_url = Config::from_env().database_url;
    let pool = db::establish_connection(&database_url);
    let app_state = Arc::new(AtomicUsize::new(0));

    let typesense_url = Config::from_env().typesense_url;
    let typesense_api_key = Config::from_env().typesense_api_key;
    let client = web::block(|| {
        Client::new()
    }).await.unwrap();
    let search_state = SearchState { client, typesense_url, typesense_api_key};

    db::run_migrations(
        &mut pool.get().expect("Unable to get db connection"),
        MIGRATIONS,
    )
    .expect("Failed to run migrations.");

    let pool_clone = pool.clone();
    let search_state_clone = search_state.clone();
    web::block(move || {
        data_sync::init_tpyesense(
            &mut pool_clone.get().expect("Unable to get db connection"),
            search_state_clone,
        );
    }).await.unwrap();

    let server = chat_server::ChatServer::new(app_state.clone()).start();
    log::debug!("Starting HTTP server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(search_state.clone()))
            .configure(routes::init)
            .wrap(cors)
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .wrap(Logger::default())
    })
    .workers(4)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
