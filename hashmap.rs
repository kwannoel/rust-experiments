// https://doc.rust-lang.org/std/collections/struct.HashMap.html
use std::collections::HashMap;

fn main() {
    // Taken from the crate docs (see above link).
    let mut book_reviews = HashMap::new();
    let huckle =
        "Adventures of Huckleberry Finn".to_string();
    let fav =
        "My favorite book.".to_string();
    book_reviews.insert(
        huckle.clone(),
        fav.clone(),
    );

    // Check if key already exists
    println!("{}", book_reviews.contains_key(&huckle)); // true
    println!("{}", book_reviews.contains_key(&fav)); // false
}
