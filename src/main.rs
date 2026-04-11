use ureq::get;
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    userId: i32,
    id: i32,
    title: String,
    completed: bool
}
// main usually returns () and Box<dyn Error> is a trait object (short for any type of error)
fn main() -> Result<(), Box< dyn Error>> { 
    // works!!!
    //"https://pokeapi.co/api/v2/pokemon/pikachu"

    let uri = "https://jsonplaceholder.typicode.com/todos/10"; 
    let body: String = get(uri)
        .call()?
        .body_mut()
        .read_to_string()?;

    let task: Task = serde_json::from_str(&body)?;

    dbg!(task.userId);
    dbg!(task.id);
    dbg!(task.title);
    dbg!(task.completed);
    dbg!(task.userId);
    Ok(())
    // need to use Serde to do things with json data... we'll be right back..
}
