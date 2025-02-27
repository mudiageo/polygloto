use crate::deck::Deck;
use crate::db::Identifiable;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Flashcard {
    pub id: usize,
    pub front: String,
    pub back: String,
    pub language: Language,
    pub media_type: MediaType,
    pub deck_id: usize,
    pub points: i8,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum MediaType {
   Text,
   Image,
   Audio,
}
impl Identifiable for Flashcard {
   fn id(&self) -> usize {
       self.id
   }
}

impl Flashcard {
    pub fn new(id: usize, front: String, back: String, language: Language, media_type: MediaType, deck_id: usize, points: i8) -> Self {
        Flashcard { id, front, back, language, media_type, deck_id, points }
    }
}


pub fn create_sample_flashcards(deck: &Deck) -> Vec<Flashcard>{
vec![
        Flashcard::new(1, "Hello".to_string(), "Hola".to_string(), Language::Spanish, MediaType::Text, 1,8),
        Flashcard::new(2, "Goodbye".to_string(), "Adiós".to_string(),  Language::Spanish, MediaType::Text, 1, 10),
    ]
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum Language {
   English,
   Urhobo,
   Edo,
   French,
   Spanish
}
