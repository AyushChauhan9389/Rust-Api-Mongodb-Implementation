use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub title: String,
    pub plot: String,
    pub rating: i32,
    pub genre: String,
    pub director: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieRequest {
    pub title: String,
    pub plot: String,
    pub rating: i32,
    pub genre: String,
    pub director: String,
}

impl TryFrom<MovieRequest> for Movie {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: MovieRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title,
            plot: value.plot,
            rating: value.rating,
            genre: value.genre,
            director: value.director,
        })
    }
}