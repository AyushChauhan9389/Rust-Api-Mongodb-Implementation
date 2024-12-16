mod models;
mod services;
mod routes;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use crate::routes::movies_route::add_movie;
use crate::services::db::Database;

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    println!("Server starting");
    let db = Database::init().await;
    println!("Database initialized");
    let db_data = Data::new(db);
    println!("Server started");
    HttpServer::new( move || App::new().app_data(db_data.clone())
        .service(hello)
        .service(add_movie))
        .bind(("localhost", 5001))
        ?.run()
        .await



}