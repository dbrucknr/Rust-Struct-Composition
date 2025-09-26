use crate::traits::repository::Repository;

pub trait Service {
    type Item;
    type Repo: Repository<Item = Self::Item>;

    // Create a way for the service trait to speak to the repo
    fn repo(&self) -> &Self::Repo;
    fn repo_mut(&mut self) -> &mut Self::Repo;

    // Default forwarding of methods
    fn find_all(&self) -> &[Self::Item] {
        self.repo().all()
    }

    fn find_one(&self, item: &Self::Item) -> Option<&Self::Item> {
        self.repo().one(item)
    }

    fn remove(&mut self, item: &Self::Item) -> Option<Self::Item> {
        self.repo_mut().remove(item)
    }

    fn update(&mut self, original: &Self::Item, updated: Self::Item) -> Option<Self::Item> {
        self.repo_mut().update(original, updated)
    }
}
