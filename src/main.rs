#[macro_use]
extern crate clap;

use clap::App;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::process::Child;
use std::fmt;

struct ShellCommand<'s> {
    name: &'s str,
    command: &'s str,
    child: Child,
}

impl<'d> fmt::Debug for ShellCommand<'d> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Command{{ name: {}, command: {} }}",
               self.name,
               self.command)
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let dir: &str = matches.value_of("directory").unwrap_or(".");
    let file: &str = matches.value_of("file").unwrap_or("Procfile");

    println!("{}/{}", dir, file);
    let contents: String = open_file(dir, file);
    println!("{}", contents);
    let commands = parse_contents(&contents);
    println!("{:?}", commands);
    let shell_commands = run_commands(commands);
    println!("{:?}", shell_commands);
    // let ecode = child.wait()
    //                  .unwrap_or_else(|e| panic!("failed to wait on child: {}", e));
    // println!("ecode {:?}", ecode);
}

fn run_commands<'f>(commands: Vec<(&'f str, &'f str)>) -> Vec<ShellCommand> {
    commands.into_iter()
            .map(|(name, command)| run_command(name, command))
            .collect::<Vec<_>>()
}

fn run_command<'f>(name: &'f str, command: &'f str) -> ShellCommand<'f> {
    let mut process = Command::new(command);
    let mut child = process.spawn()
                           .unwrap_or_else(|e| panic!("failed to execute child: {}", e));
    ShellCommand {
        name: name,
        command: command,
        child: child,
    }
}

fn parse_contents<'p>(contents: &'p String) -> Vec<(&'p str, &'p str)> {
    contents.lines()
            .map(|s| s.split(':').collect::<Vec<&str>>())
            .map(|a| (a[0], a[1]))
            .collect::<Vec<(&str, &str)>>()
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
