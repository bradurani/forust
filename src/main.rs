#[macro_use]
extern crate clap;
use clap::App;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::collections::HashMap;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let dir: &str = matches.value_of("directory").unwrap_or(".");
    let file: &str = matches.value_of("file").unwrap_or("Procfile");
    println!("{}/{}", dir, file);
    let contents: String = open_file(dir, file);
    println!("{}", contents);
    let hm = parse_contents(&contents);
    println!("{:?}", hm);
}
fn parse_contents(contents: &str) -> HashMap<&str, &str> {
    let lines = contents.lines()
                        .collect::<Vec<&str>>();
    let iter = lines.iter();
    let collection = iter.map(|s| s.split(':').collect::<Vec<&str>>())
                         .collect::<Vec<Vec<&str>>>();
    let iter2 = collection.iter();

    let mut hm: HashMap<&str, &str> = HashMap::new();
    for vector in iter2 {
        hm.insert(vector[0], vector[1]);
    }
    hm
}

fn run_command(cmd: &str) {
    let mut process = Command::new(cmd);
    let mut child = process.spawn()
                           .unwrap_or_else(|e| panic!("failed to execute child: {}", e));
    let id = child.id();
    let ecode = child.wait()
                     .unwrap_or_else(|e| panic!("failed to wait on child: {}", e));
    println!("ecode {:?}", ecode);
    id;
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
        Ok(_) => s,
    }

}
