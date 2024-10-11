use sqlite;
use sqlite::OpenFlags;

fn main_fail() {
    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute(
            "CREATE TABLE users (name TEXT NOT NULL, age INTEGER NOT NULL)",
        )
        .unwrap();

    connection
        .execute("INSERT INTO users (name, age) VALUES ('Alice', 42)").unwrap();

    let mut statement = connection
        .prepare("SELECT name, age FROM users WHERE age > 30")
        .unwrap();
    let cursor = statement.iter();
    for row in cursor {
        println!("{:?}", row);
    }

    let another_connection = sqlite::open(":memory:").unwrap();
    another_connection
        .iterate("SELECT name, age FROM users WHERE age > 30", |row| {
            println!("{:?}", row);
            true
        }).unwrap();
}

fn main() {
    // let connection = sqlite::open("file::memory:?cache=shared").unwrap();
    let connection = sqlite::Connection::open_with_flags("file::memory:?cache=shared", OpenFlags::new().with_create().with_read_write().with_uri()).unwrap();
    connection
        .execute(
            "CREATE TABLE users (name TEXT NOT NULL, age INTEGER NOT NULL)",
        )
        .unwrap();

    connection
        .execute("INSERT INTO users (name, age) VALUES ('Alice', 42)").unwrap();

    let mut statement = connection
        .prepare("SELECT name, age FROM users WHERE age > 30")
        .unwrap();
    let cursor = statement.iter();
    for row in cursor {
        println!("{:?}", row);
    }

    let another_connection = sqlite::Connection::open_with_flags("file::memory:?cache=shared", OpenFlags::new().with_create().with_read_write().with_uri()).unwrap();
    another_connection
        .iterate("SELECT name, age FROM users WHERE age > 30", |row| {
            println!("{:?}", row);
            true
        }).unwrap();
}