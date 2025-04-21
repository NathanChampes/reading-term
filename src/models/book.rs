use chrono::{DateTime, Utc};
use std::fmt;
use uuid::Uuid;

// Trois cas de statut
pub enum ReadingStatus {
    ToRead,
    Reading,
    Read,
}

// Bon la je défini une structure livre pour que ce soit plus simple
pub struct Book {
    pub id: Uuid,
    pub name: String,
    // Il peut y avoir un 1...N auteurs qui participent
    pub authors_id: Vec<Uuid>, // Ici on charge également un indentifiant pour que ce soit léger
    pub state: ReadingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Book {
    pub fn new(name: String, authors_id: Vec<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            authors_id: authors_id,
            state: ReadingStatus::ToRead,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_status(&mut self, status: ReadingStatus) {
        self.state = status;
        self.updated_at = Utc::now();
    }

    pub fn add_author(&mut self, authors_id: Uuid) {
        if !self.authors_id.contains(&authors_id) {
            self.authors_id.push(authors_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_author(&mut self, authors_id: Uuid) {
        if self.authors_id.contains(&authors_id) {
            // a trouver parce que j'ai pas compris la
            self.updated_at = Utc::now();
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, name: {}, authors_id: {}, state: {}, created_at: {}, updated_at: {}",
            self.id, self.name, self.authors_id, self.state, self.created_at, self.updated_at
        );
    }
}
