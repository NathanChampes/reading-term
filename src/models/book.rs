use chrono::{DateTime, Utc};
use std::fmt;
use uuid::Uuid;

use crate::models::Author;

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
    pub fn new(name: String, mut authors: Vec<&mut Author>) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v4();

        // Pour chaque auteur passé en paramètre d'initialisation, on ajoute le livre
        for author in &mut authors {
            author.add_book(id);
        }

        // pour chaque auteur on récupère l'id dans la collection
        let authors_id = authors.iter().map(|author| author.id).collect();
        Self {
            id,
            name,
            authors_id,
            state: ReadingStatus::ToRead,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_status(&mut self, status: ReadingStatus) {
        self.state = status;
        self.updated_at = Utc::now();
    }

    pub fn add_author(&mut self, author: &mut Author) {
        if !self.authors_id.contains(&author.id) {
            self.authors_id.push(author.id);
            self.updated_at = Utc::now();

            // En gros j'ai modifié la façon dont on gère l'ajout parce que je veux que les liens
            // soient bidirectionnels
            author.add_book(self.id);
        }
    }

    pub fn remove_author(&mut self, author: &mut Author) {
        if self.authors_id.contains(&author.id) {
            self.authors_id.retain(|&id| id != author.id);
            self.updated_at = Utc::now();

            // Du coup forcément ici aussi on appel une méthode de la structure autheur pour la
            // synchronisation bidirectionnelle
            author.remove_book(self.id);
        }
    }
}

// La c'est l'implémentation du display pour l'enum
impl fmt::Display for ReadingStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match self {
            ReadingStatus::ToRead => "To Read",
            ReadingStatus::Reading => "Reading",
            ReadingStatus::Read => "Read",
        };
        write!(f, "{}", status)
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // On récupère tous les auteurs et on les join genre : victor hugo, gros con, ect
        let authors = self
            .authors_id
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "id: {}, name: {}, authors_id: [{}], state: {}, created_at: {}, updated_at: {}",
            self.id, self.name, authors, self.state, self.created_at, self.updated_at
        )
    }
}
