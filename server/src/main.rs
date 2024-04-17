#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
use mongodb::{options::ClientOptions, sync::{Client, Database}};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Method::{Get, Post};
use rocket::ignite;

#[path= "controller/db_request.rs"]
pub mod db_request;
#[path="controller/document_update.rs"]
pub mod document_update;

pub struct MongoDB {
    db: Database,
}

impl MongoDB {
    // New MongoDB Connection:
    pub fn new() -> Result<MongoDB, mongodb::error::Error> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;
        client_options.app_name = Some("Project3Server".to_string());
        let client = Client::with_options(client_options)?;
        // Connection:
        let db = client.database("ServerDB"); // Change at later stage:

        Ok(MongoDB { db })
    }
}

fn get_all_route() -> Vec<rocket::Route> {
    routes![
        db_request::new_user,
        db_request::authentication_verify,
        db_request::get_scores,
        db_request::update_score,
    ]
}

fn main() {
    // Initialize the rocket:
    let cors_options = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Get, Post].into_iter().map(From::from).collect())
        .allow_credentials(true);
    let routes = get_all_route();
    // Ignite the rocket:
    ignite().attach(cors_options.to_cors().unwrap()).mount("/", routes).launch();
}
