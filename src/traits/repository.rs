pub trait Repository {
    type Item;
    type RepoError;

    // Borrow-Based API
    fn all(&self) -> Result<&[Self::Item], Self::RepoError>;
    fn one(&self, item: &Self::Item) -> Result<&Self::Item, Self::RepoError>;
    fn remove(&mut self, item: &Self::Item) -> Result<Self::Item, Self::RepoError>;
    fn update(
        &mut self,
        original: &Self::Item,
        updated: Self::Item,
    ) -> Result<Self::Item, Self::RepoError>;
}
