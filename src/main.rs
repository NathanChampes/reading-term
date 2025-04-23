mod models;

use crate::models::{Author, Book};

fn main() {
    let author1 = Author::new("Victor Hugo".to_string());
    println!("test display auteur : {}", author1);

    // Je découvre juste l'utilisation des trucs
    let mut vec = Vec::new();
    vec.push(author1.id);
    let book1 = Book::new("Les misérables".to_string(), vec);
    println!("test display livre : {}", book1);
}
