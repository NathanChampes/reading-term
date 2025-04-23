mod models;

use crate::models::{Author, Book};

fn main() {
    let author1 = Author::new("Victor Hugo".to_string());
    println!("test display auteur : {}", author1);

    let book1 = Book::new("Les misérables".to_string(), vec![author1.id]);
    println!("test display livre : {}", book1);
}
