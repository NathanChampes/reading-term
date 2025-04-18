use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Author {
    pub id: Uuid,
    pub name: String,
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
}
