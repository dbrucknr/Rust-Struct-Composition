use std::error::Error;

use crate::traits::repository::Repository;

pub trait Service {
    type Item;
    type Repo: Repository<Item = Self::Item>;
    type ServiceError: From<<Self::Repo as Repository>::RepoError> + Error;

    // Create a way for the service trait to speak to the repo
    fn repo(&self) -> &Self::Repo;
    fn repo_mut(&mut self) -> &mut Self::Repo;

    // Default forwarding of methods
    fn find_all(&self) -> Result<&[Self::Item], Self::ServiceError> {
        self.repo().all().map_err(Into::into)
    }

    fn find_one(&self, item: &Self::Item) -> Result<&Self::Item, Self::ServiceError> {
        self.repo().one(item).map_err(Into::into)
    }

    fn remove(&mut self, item: &Self::Item) -> Result<Self::Item, Self::ServiceError> {
        self.repo_mut().remove(item).map_err(Into::into)
    }

    fn update(
        &mut self,
        original: &Self::Item,
        updated: Self::Item,
    ) -> Result<Self::Item, Self::ServiceError> {
        self.repo_mut()
            .update(original, updated)
            .map_err(Into::into)
    }
}
