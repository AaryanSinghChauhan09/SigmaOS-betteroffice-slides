#[derive(Debug, Clone)]
pub struct CollabSlide { pub index: usize, pub title: String }

#[derive(Debug, Default)]
pub struct CollabDeck { slides: Vec<CollabSlide> }
impl CollabDeck {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, title: impl Into<String>) {
        let idx = self.slides.len();
        self.slides.push(CollabSlide { index: idx, title: title.into() });
    }
    pub fn count(&self) -> usize { self.slides.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_deck() {
        let mut d = CollabDeck::new();
        d.add("Overview");
        assert_eq!(d.count(), 1);
    }
}
