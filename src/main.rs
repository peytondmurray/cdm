use toml::de;
use serde::Deserialize;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct Config {
    platforms: Vec<String>,
    templates: HashMap<String, String>
}

fn load_config(filename: Option<&str>) -> Result<Config, io::Error> {
    let mut f = File::open(&filename.unwrap_or("/home/pdmurray/dotfiles/cdm.toml"))?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(toml::from_str::<Config>(&buf)?)
}

fn load_ignore(filename: Option<&str>) -> Result<Ignore, io:Error>  {

}

fn main() {
    let config = load_config(None).expect("No config parsed.");
    println!("{:#?}", config);
}
