use std::error::Error;
use std::fs;
use std::io::{BufReader, Cursor};
                                                            
use serde::{Serialize, Deserialize};

use ureq::get;

use viuer::{print_from_file, Config};

use clap::Parser;

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
    #[serde(rename = "back_shiny")]
    pub back_shiny: String,
    #[serde(rename = "front_default")]
    pub front_default: String,
    #[serde(rename = "front_shiny")]
    pub front_shiny: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cries {
    pub latest: String,
    pub legacy: String,
}

// define a struct of args for user to pass in... 
// bare minimum is sound-toggle and name of mon
#[derive(Parser)]
struct Cli {
    /// Name of mon to poke
    mon: String,

    /// Bool toggle to enable sound (sound disabled by default) 
    #[arg(short, long, default_value_t = false)]
    sound: bool,
}
// main usually returns () and Box<dyn Error> is a trait object (short for any type of error)
fn main() -> Result<(), Box< dyn Error>> { 
    
    let args = Cli::parse();

    let mut url =  String::from("https://pokeapi.co/api/v2/pokemon/");

    url.push_str(&args.mon);

    let body: String = get(&url)
        .call()?
        .body_mut()
        .read_to_string()?;

    let pokemon : Pokemon = serde_json::from_str(&body)?;

    let _ = show_image(&pokemon.sprites.front_default, &args.mon);
    if args.sound {
        let _ = play_audio(&pokemon.cries.latest);
    }

    Ok(())
}

// found out that terminals use different rendering engines, so transition to use inline images 
// TODO: cache results so dont need to use network oftern 
fn show_image(url: &str, name: &str) -> Result<(), Box<dyn Error>> { 
    // if the file exists, go straight to rendering it
    let path = format!("sprites/{}.png", name);

    if !fs::exists(&path)? {
        fs::create_dir_all("sprites/")?;
        let sprite = get(url).call()?.body_mut().read_to_vec()?;
        fs::write(&path, &sprite)?;
    }

    let conf = Config {
        ..Default::default()
    };

    print_from_file(&path, &conf).expect("Image printing failed");
    Ok(())
}

// TODO: cache results so dont need network
fn play_audio(url: &str) -> Result<(), Box< dyn Error>>{

    // TODO: download and cache it, 
    // TODO: if the file exists, go straight to playing it 
    // load the sound from thr url, can either download it or load in into bytes
    let cursor =  Cursor::new(get(url)
        .call()?
        .body_mut()
        .read_to_vec()?);


    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the sink_handle is dropped.
    let mut sink_handle = rodio::DeviceSinkBuilder::open_default_sink()?;

    sink_handle.log_on_drop(false);
    let audio_file = BufReader::new(cursor);
    // Note that the playback stops when the player is dropped
    let _player = rodio::play(sink_handle.mixer(), audio_file).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    
    // length of sound should be based on length of file
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}
