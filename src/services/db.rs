use std::env;
use futures_util::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use crate::models::movie::Movie;

pub struct Database{
    movie: Collection<Movie>
}


impl Database{
    pub async fn init() -> Self{
        let uri =  match env::var("MONGO_URI"){
            Ok(uri) => uri,
            Err(_) => panic!("MONGO_URI not found")
        };

        let client  = Client::with_uri_str(uri).await.unwrap();
        let db =  client.database("movie");

        let movie = db.collection("movie");
        Database{
            movie
        }

    }

    pub async fn insert_movie(&self, movie: Movie) -> Result<InsertOneResult, Error>  {
        let result = self
            .movie
            .insert_one(movie)
            .await
            .ok()
            .expect("Failed to insert movie");

        Ok(result)
    }
    pub async fn get_all_movies(&self) -> Result<Vec<Movie>, Error> {
        let filter = doc! {};
        let cursor = self.movie.find(filter).await?;
        let movies: Vec<Movie> = cursor.try_collect().await?;
        Ok(movies)
    }


}