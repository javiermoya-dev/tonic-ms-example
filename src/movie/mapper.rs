use crate::movie::entity::Movie;
use crate::movie::grpc_movie::MovieItem;
//use crate::movie::grpc_movie::{CreateItemRequest, UpdateItemRequest};


impl From<Movie> for MovieItem {
    fn from(movie: Movie) -> Self {

        MovieItem {
            id: movie.id,
            title: movie.title,
            year: movie.year,
            genre: movie.genre,
            rating: movie.rating,
            cast: movie.cast,
            image: movie.image,
        }
    }
  }
  