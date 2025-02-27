mod flashcard;
use flashcard::{ Language, MediaType, Flashcard};

mod deck;
use deck::{Deck, create_deck};

mod db;
use db::{Store, create_store};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn flashcards() -> Vec<Flashcard> {

 let mut deck_store: Store<Deck> = create_store::<Deck>("decks");
 
 let deck: Deck = create_deck(1, String::from("new deck"), "super deck".to_string(), Language::Urhobo);
     
   deck_store.insert(deck.clone());
   
   let my_deck = deck_store.get(1);
   
   println!("Question: {}\nAnswer: {}\nPoints to earn: {}\n", my_deck.name, my_deck.description, my_deck.id);


 let mut flashcard_store: Store<Flashcard> = create_store::<Flashcard>("flashcards");
  
 let flashcard = Flashcard::new(3, "Mavọ".to_string(), "How are you?".to_string(), Language::Urhobo, MediaType::Text, 1, 10);
 
 flashcard_store.insert(flashcard.clone());
    
    
    let flashcards = flashcard_store.get_all();
    //  for flashcard in &flashcards {
    //      println!("Question: {}\nAnswer: {}\nPoints to earn: {}\n", flashcard.front, flashcard.back, flashcard.points);
    //  }
     flashcards.clone()
 }


 #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, flashcards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
