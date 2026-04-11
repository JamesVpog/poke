use ureq::get;
use std::error::Error;

fn main() -> Result<(), Box< dyn Error>> {
    let body: String = get("https://pokeapi.co/api/v2/pokemon/pikachu")
        .call()?
        .body_mut()
        .read_to_string()?;

    println!("{}", body);
    Ok(())
}
