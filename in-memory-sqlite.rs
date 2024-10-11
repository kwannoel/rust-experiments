use sqlite;
use sqlite::OpenFlags;

/// This fails because:
/// 1. sqlite in-memory database is not shared between connections.
/// 2. So the second connection is not able to see the data inserted by the first connection.
/// 3. We also need to interpret the string as URI to enable shared cache.
fn main_fail_1() {
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

/// If the connection is dropped, the in-memory database is destroyed.
fn main_fail_2() {
    {
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
    }

    let another_connection = sqlite::Connection::open_with_flags("file::memory:?cache=shared", OpenFlags::new().with_create().with_read_write().with_uri()).unwrap();
    another_connection
        .iterate("SELECT name, age FROM users WHERE age > 30", |row| {
            println!("{:?}", row);
            true
        }).unwrap();
}

thread_local! {
    pub static __CONNECTION: sqlite::Connection = sqlite::Connection::open_with_flags("file::memory:?cache=shared", OpenFlags::new().with_create().with_read_write().with_uri()).unwrap();
}

fn main() {
    // initialize the connection.
    __CONNECTION.with(|connection| {
    });
    {
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
    }

    // even if dropped, it is still present.

    let another_connection = sqlite::Connection::open_with_flags("file::memory:?cache=shared", OpenFlags::new().with_create().with_read_write().with_uri()).unwrap();
    another_connection
        .iterate("SELECT name, age FROM users WHERE age > 30", |row| {
            println!("{:?}", row);
            true
        }).unwrap();
}