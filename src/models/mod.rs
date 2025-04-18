// Export les modules
pub mod author;
pub mod book;
pub mod note;

// Re-export des structures principales pour faciliter leur import
pub use author::Author;
pub use book::{Book, ReadingStatus};
pub use note::Note;
