use tonic::{Request, Response, Status};
use crate::movie::interactor::MovieInteractor;

use crate::movie::grpc_movie;
use crate::movie::grpc_movie::{MovieResponse , MovieRequest};  
use crate::movie::grpc_movie::movie_server::Movie;
use crate::movie::grpc_movie::MovieItem;

// #[derive(Debug, Default)]
// pub struct MovieService {
// }

pub struct MovieService {
    interactor: Box<dyn MovieInteractor + Send + Sync>,
}

impl MovieService {
    pub fn new(interactor: Box<dyn MovieInteractor + Send + Sync>) -> impl Movie {
        MovieService { interactor }
    }
}

#[tonic::async_trait]
impl Movie for MovieService {
    async fn get_movies(
        &self,
        _request: Request<MovieRequest>,
    ) -> Result<Response<MovieResponse>, Status> {

        match self.interactor.get_all_movies().await {
            Ok(movies) => {
                let movies_grpc: Vec<MovieItem> = movies.into_iter().map(|movie| movie.into()).collect();
                let response = MovieResponse { movies: movies_grpc.into() };
                Ok(Response::new(response))
            }
            
            Err(_) => Err(Status::internal("Failed to fetch items")),
        }

        /*let mut movies = Vec::new();
        movies.push(grpc_movie::MovieItem {
            id: 1,
            title: "Matrix".to_string(),
            year: 1999,
            genre: "Sci-Fi".to_string(),
            rating: "8.7".to_string(),
            star_rating: "4.8".to_string(),
            runtime: "136".to_string(),
            cast: "Keanu Reeves, Laurence Fishburne".to_string(),
            image: "http://image.tmdb.org/t/p/w500//aOIuZAjPaRIE6CMzbazvcHuHXDc.jpg".to_string(),
        });
        movies.push(grpc_movie::MovieItem {
            id: 2,
            title: "Spider-Man: Across the Spider-Verse".to_string(),
            year: 2023,
            genre: "Animation".to_string(),
            rating: "9.7".to_string(),
            star_rating: "4.9".to_string(),
            runtime: "136".to_string(),
            cast: "Donald Glover".to_string(),
            image: "http://image.tmdb.org/t/p/w500//8Vt6mWEReuy4Of61Lnj5Xj704m8.jpg".to_string(),
        });
        movies.push(grpc_movie::MovieItem {
            id: 3,
            title: "Her".to_string(),
            year: 2013,
            genre: "Drama".to_string(),
            rating: "8.7".to_string(),
            star_rating: "4.1".to_string(),
            runtime: "136".to_string(),
            cast: "Joaquin Phoenix".to_string(),
            image: "http://image.tmdb.org/t/p/w500//eCOtqtfvn7mxGl6nfmq4b1exJRc.jpg".to_string(),
        });

        let reply = grpc_movie::MovieResponse { movies: movies };

        Ok(Response::new(reply))*/
    }
}
