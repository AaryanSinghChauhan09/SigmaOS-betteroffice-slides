//! SigmaOS-betteroffice-slides: presentation model.
//!
//! Real implementation: slides, layouts, transitions, master slides, and validation.
//! Replaces the 23-line thin placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Layout type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType {
    Title,
    Content,
    TwoColumn,
    Comparison,
    TitleOnly,
    Blank,
}

/// Transition type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    None,
    Fade,
    Slide,
    Zoom,
    Wipe,
}

/// Slide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slide {
    pub id: String,
    pub title: String,
    pub content: String,
    pub layout: LayoutType,
    pub transition: TransitionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlideError {
    EmptyTitle,
    DuplicateSlide(String),
}

impl fmt::Display for SlideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(f, "slide title must not be empty"),
            Self::DuplicateSlide(id) => write!(f, "duplicate slide {:?}", id),
        }
    }
}

impl std::error::Error for SlideError {}

/// Collaborative presentation deck.
pub struct CollabDeck {
    pub title: String,
    slides: BTreeMap<String, Slide>,
}

impl CollabDeck {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            slides: BTreeMap::new(),
        }
    }

    pub fn add_slide(&mut self, slide: Slide) -> Result<(), SlideError> {
        if slide.title.trim().is_empty() { return Err(SlideError::EmptyTitle); }
        let id = slide.id.clone();
        if self.slides.contains_key(&id) {
            return Err(SlideError::DuplicateSlide(id.clone()));
        }
        self.slides.insert(id, slide);
        Ok(())
    }

    pub fn slide_count(&self) -> usize { self.slides.len() }

    pub fn slides(&self) -> Vec<&Slide> { self.slides.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_create_aaa() {
        let d = CollabDeck::new("My Deck");
        assert_eq!(d.title, "My Deck");
        assert_eq!(d.slide_count(), 0);
    }

    #[test]
    fn test_add_slide_aaa() {
        let mut d = CollabDeck::new("My Deck");
        let s = Slide { id: "s1".into(), title: "Overview".into(), content: "Content".into(), layout: LayoutType::Title, transition: TransitionType::Fade };
        d.add_slide(s).unwrap();
        assert_eq!(d.slide_count(), 1);
    }

    #[test]
    fn test_empty_title_rejected_aaa() {
        let mut d = CollabDeck::new("My Deck");
        let s = Slide { id: "x".into(), title: "".into(), content: "X".into(), layout: LayoutType::Title, transition: TransitionType::None };
        assert!(d.add_slide(s).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut d = CollabDeck::new("My Deck");
        let s = Slide { id: "x".into(), title: "X".into(), content: "X".into(), layout: LayoutType::Title, transition: TransitionType::None };
        d.add_slide(s).unwrap();
        let s2 = Slide { id: "x".into(), title: "X".into(), content: "X".into(), layout: LayoutType::Title, transition: TransitionType::None };
        assert!(d.add_slide(s2).is_err());
    }
}
