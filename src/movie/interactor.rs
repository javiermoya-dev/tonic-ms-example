use crate::movie::entity::{Movie, MovieDTO};
use crate::movie::repository::MovieRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tonic::async_trait]
pub trait MovieInteractor {
    async fn get_all_movies(&self) -> Result<Vec<Movie>, Error>;
}

pub struct MovieInteractorImpl {
    repository: Box<dyn MovieRepository + Send + Sync>,
}

impl MovieInteractorImpl {
    pub fn new(repository: Box<dyn MovieRepository + Send + Sync>) -> Box<dyn MovieInteractor + Send + Sync> {
        Box::new(MovieInteractorImpl { repository })
    }
}

#[tonic::async_trait]
impl MovieInteractor for MovieInteractorImpl {
    async fn get_all_movies(&self) -> Result<Vec<Movie>, Error> {
        self.repository.get_all_movies().await
    }
}
