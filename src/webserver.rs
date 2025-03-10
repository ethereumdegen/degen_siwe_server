 
use degen_siwe_server::app_state::AppConfig;
use degen_siwe_server::app_state::AppState;
use degen_sql::db::postgres::postgres_db::Database;
use dotenvy::dotenv;
use std::sync::Arc;
use tokio::io;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

mod controllers;

use crate::controllers::web_controller::WebController;
 
use controllers::session_controller::SessionController; 


/*

Serves the HTTP api

See 'controllers' for routes and functions

*/


#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    // Initialize the logger
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info"); // Adjust as per your needs
    env_logger::init();

    println!("connecting to db.");
 

    let db_conn_url = std::env::var("DB_CONN_URL").expect(" DB_CONN_URL must be set in env ");


    let service_name = std::env::var("SERVICE_NAME").expect(" SERVICE_NAME must be set in env ");

    let app_config = Arc::new( AppConfig {

        service_name 
    } );

    let database = Arc::new(Database::new(db_conn_url, None).unwrap());

    println!("connected to db.");

    //setup and launch the http server
    HttpServer::new(move || {
        let cors = Cors::default()
            //  .allowed_origin("http://localhost:3000")
            // .allowed_origin("http://localhost:8080")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Authorization", "Accept", "Content-Type"])
            .supports_credentials()
            .max_age(3600);

        let app_state = AppState {
            database: Arc::clone(&database), 

            app_config: Arc::clone(&app_config), 
        };

        App::new()
            .app_data(Data::new(app_state)) // Clone your db connection or use Arc
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default()) // Enable logger middleware
            .configure(SessionController::config) // log in with eth
            } )
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
