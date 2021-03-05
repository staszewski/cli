use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    userId: i32,
    id: i32,
    title: String,
    completed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp =
        reqwest::blocking::get("https://jsonplaceholder.typicode.com/todos/1")?.json::<User>()?;
    println!("{:?}", resp);
    Ok(())
}
