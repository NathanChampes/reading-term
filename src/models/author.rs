use chrono::{DateTime, Utc};
use uuid::Uuid;

// On défini la structure des auteurs
pub struct Author {
    pub id: Uuid,
    pub name: String,
    // les auteurs peuvent avoir 1...N livres
    pub books_id: Vec<Uuid>, // vecteur de ids de books pour être plus léger
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Author {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            books_id: Vec::<Uuid>::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_book(&mut self, book_id: Uuid) {
        if !self.books_id.contains(&book_id) {
            self.books_id.push(book_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_book(&mut self, book_id: Uuid) {
        if self.books_id.contains(&book_id) {
            // La il faut vraiment que je regarde pour le remove
            self.updated_at = Utc::now();
        }
    }

    pub fn display(&mut self) {
        println!("{}, {}", self.id, self.name);
    }
}
