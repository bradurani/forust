#[macro_use]
extern crate clap;
use clap::App;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let dir: &str = matches.value_of("directory").unwrap_or(".");
    let file: &str = matches.value_of("file").unwrap_or("Procfile");
    println!("{:?}/{:?}", dir, file);
    let contents = open_file(dir, file);
}

fn open_file(dir: &str, file: &str) -> String {
    let path = Path::new(dir).join(file);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    s
}
