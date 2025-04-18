use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum ReadingStatus {
    ToRead,
    Reading,
    Read,
}

pub struct Book {
    pub id: Uuid,
    pub name: String,
    pub authorsId: Vec<Uuid>, // Ici on charge également un indentifiant pour que ce soit léger
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
            authorsId: authors_id,
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
        if !self.authorsId.contains(&authors_id) {
            self.authorsId.push(authors_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_author(&mut self, authors_id: Uuid) {
        if self.authorsId.contains(&authors_id) {
            // a trouver parce que j'ai pas compris la
            self.updated_at = Utc::now();
        }
    }
}
