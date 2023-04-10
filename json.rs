use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
    snail_case: u8,
    camelCase: u8,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "snail_case": 1,
            "camelCase": 2
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    // println!("Please call {} at the number {}", p.name, p.phones[0]);
    println!(
        "Snail case works {}, Camel case works {}",
        p.snail_case, p.camelCase
    );

    Ok(())
}

fn main() -> Result<()> {
    typed_example()
}
