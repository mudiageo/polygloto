 #[derive(Clone)]
 pub struct Flashcard {
     pub question: String,
     pub answer: String,
     pub points: i8,
 }

 impl Flashcard {
     pub fn new(question: String, answer: String, points: i8) -> Self {
         Flashcard { question, answer, points }
     }
 }

 pub fn create_sample_flashcards() -> Vec<Flashcard> {
     vec![
         Flashcard::new("Hello".to_string(), "Hola".to_string(), 8),
         Flashcard::new("Goodbye".to_string(), "Adiós".to_string(), 10),
     ]
 }