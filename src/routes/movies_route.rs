use actix_web::{post, HttpResponse};

use actix_web::web::{Data, Json};
use crate::models::movie::{Movie, MovieRequest};
use crate::services::db::Database;

#[post("/add_movie")]
pub async fn add_movie(db: Data<Database>, request: Json<MovieRequest>) -> HttpResponse{
    match db.insert_movie(Movie::try_from(MovieRequest {
        title: request.title.clone(),
        plot: request.plot.clone(),
        rating: request.rating,
        genre: request.genre.clone(),
        director: request.director.clone(),
    }).expect("Error")
    ).await{
        Ok(Movie) => HttpResponse::Ok().json(Movie),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add movie")
    }
}
