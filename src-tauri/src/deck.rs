use crate::flashcard::Language ;
use crate::db::Identifiable;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
 pub struct Deck {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub language: Language,
}

impl Identifiable for Deck {
    fn id(&self) -> usize {
        self.id
    }
 }
 
impl Deck {
    pub fn new(id: usize, name: String, description: String, language: Language) -> Deck{
        Deck {id, name, description, language}
    }
 }
 
 
pub fn create_deck(id: usize, name: String, description: String, language: Language) -> Deck {
  Deck::new(id, name, description, language)
}
