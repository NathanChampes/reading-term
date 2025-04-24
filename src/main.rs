mod models;

use crate::models::{Author, Book};

fn main() {
    let mut author1 = Author::new("Victor Hugo".to_string());
    println!("test display auteur : {}", author1);

    let book1 = Book::new("Les misérables".to_string(), 2598, vec![&mut author1]);
    println!("test display livre : {}", book1);

    println!("Est-ce que du coup l'auteur a le livre ? : {}", author1);
}
