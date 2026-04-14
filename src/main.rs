use ureq::get;
use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::io::{BufReader, Cursor};

// only use the cries and sprites url to render later
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Pokemon { 
    pub cries: Cries,
    pub sprites: Sprites,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprites {
    #[serde(rename = "back_default")]
    pub back_default: String,
    #[serde(rename = "back_female")]
    pub back_female: Value,
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "back_shiny_female")]
    pub back_shiny_female: Value,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_female")]
    pub front_female: Value,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cries {
    pub latest: String,
    pub legacy: String,
}
// main usually returns () and Box<dyn Error> is a trait object (short for any type of error)
fn main() -> Result<(), Box< dyn Error>> { 
    // works!!!
    let uri = "https://pokeapi.co/api/v2/pokemon/pikachu"; 
    let body: String = get(uri)
        .call()?
        .body_mut()
        .read_to_string()?;

    // WARNING: SOUND!!!
    let pokemon : Pokemon = serde_json::from_str(&body)?;

    let _ = play_audio(&pokemon.cries.latest);

    Ok(())
}

fn play_audio(url: &str) -> Result<(), Box< dyn Error>>{
    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the sink_handle is dropped.
    let sink_handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");

    // load the sound from thr url, can either download it or load in into bytes
    let cursor =  Cursor::new(get(url)
        .call()?
        .body_mut()
        .read_to_vec()?);


    let audio_file = BufReader::new(cursor);
// Note that the playback stops when the player is dropped
    let _player = rodio::play(sink_handle.mixer(), audio_file).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}
