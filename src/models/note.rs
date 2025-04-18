pub struct Note {
    pub id: Uuid,
    pub book_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
