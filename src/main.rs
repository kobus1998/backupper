extern crate termion;
extern crate walkdir;

use walkdir::WalkDir;
use termion::{color, style};

use std::env::{self, Args};
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

struct Settings<'a> {
    pub options: Vec<String>,
    pub arguments: HashMap<String, String>,
    pub start: &'a Path,
    pub end: &'a Path
}
impl <'a> Settings <'a> {

    pub fn new() -> Settings<'a> {

        let settings = Settings {
            options: Settings::get_options(),
            arguments: Settings::get_arguments(),
            start: Path::new(""),
            end: Path::new("")
        };

        settings.display_arguments();
        println!("");
        settings.display_options();

        settings
    }

    pub fn display_options(&self) {
        println!("Options");
        for option in &self.options {
            println!("{}", option);
        }
    }

    pub fn display_arguments(&self) {
        println!("Arguments");
        for (key, value) in &self.arguments {
            println!("{}: {}", key, value);
        }
    }

    pub fn get_options() -> Vec<String> {
        env::args()
            .into_iter()
            .filter(|a| a.starts_with("-"))
            .collect()
    }

    pub fn get_arguments() -> HashMap<String, String> {
        let args: Vec<String> = env::args()
            .into_iter()
            .filter(|a| !a.starts_with("-"))
            .map(|a| a.to_string())
            .collect();

        let mut from = "";
        let mut to = "";

        match args.get(1) {
            Some(x) => from = x,
            None => println!("Sorry, this vector is too short.")
        }

        match args.get(2) {
            Some(x) => to = x,
            None => println!("Sorry, this vector is too short.")
        }

        let mut mapped = HashMap::new();

        mapped.insert(String::from("from"), from.to_string());
        mapped.insert(String::from("to"), to.to_string());

        mapped
    }

}

pub struct Backup {
    // settings: Settings
}
impl Backup {

    pub fn new() -> Backup {
        Backup {
            // settings: Settings::new()
        }
    }

    pub fn create(name: &String, data: String) {
        let mut file = File::create(name).unwrap();
        file.write_all(data.to_string().as_bytes()).unwrap();
    }

    pub fn read(path: &Path) -> String {
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);

        contents
    }

    pub fn copy(start: &Path, dir: &String) {
        let name = start.file_name().unwrap().to_str().unwrap();
        fs::create_dir_all(str::replace(dir, name, ""));
        Backup::create(dir, Backup::read(start));
    }

    pub fn run(&self, start: &Path, end: &Path) {
        let start_name: &str = start.to_str().unwrap();
        let end_name: &str = end.to_str().unwrap();

        for entry in WalkDir::new(start_name).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.to_str().unwrap();

            if !path.is_dir() {
                let path_to = str::replace(path.to_str().unwrap(), start_name, end_name);
                Backup::copy(path, &path_to.to_string());
            }
        }
    }
}




fn main() {
    let start = Path::new("/home/superuser/Documents/bu-from");
    let end = Path::new("/home/superuser/Documents/bu-to");

    let settings = Settings::new();

    // let backup = Backup::new();

    // Backup::run_through_directories(start, end);
}
