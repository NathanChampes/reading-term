mod models;

use crate::models::{Author, Book, Note};

fn main() {
    let mut author1 = Author::new("Victor Hugo".to_string());
    author1.display();

    // Je découvre juste l'utilisation des trucs
    let mut vec = Vec::new();
    vec.push(author1.id);
    let _book1 = Book::new("Digital minimalism".to_string(), vec);
}
