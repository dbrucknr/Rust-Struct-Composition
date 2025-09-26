pub trait Repository {
    type Item;

    // Borrow-Based API
    fn all(&self) -> &[Self::Item];
    fn one(&self, item: &Self::Item) -> Option<&Self::Item>;
    fn remove(&mut self, item: &Self::Item) -> Option<Self::Item>;
    fn update(&mut self, original: &Self::Item, updated: Self::Item) -> Option<Self::Item>;
}
