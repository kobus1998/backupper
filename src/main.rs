extern crate termion;
extern crate walkdir;

use walkdir::WalkDir;

use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub struct Backup;
impl Backup {

    pub fn create(name: String, data: String) {
        let mut file = File::create(name).unwrap();
        file.write_all(data.to_string().as_bytes()).unwrap();
    }

    pub fn read(path: &Path) -> String {
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);

        contents
    }

    pub fn copy(start: &Path, dir: String) {
        let name = start.file_name().unwrap().to_str().unwrap();
        let content = Backup::read(start);
        Backup::create(name.to_string(), content);
    }

    pub fn run_through_directories(start: &Path, end: &Path) {
        let start_name: String = start.display().to_string();
        let end_name: String = end.display().to_string();

        for entry in WalkDir::new(start_name).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let name: String = path.file_name().unwrap().to_str().unwrap().to_string();
            let parent: String = path.parent().unwrap().to_str().unwrap().to_string();
            

            if !path.is_dir() {
                println!("{}", parent);
            }
        }
    }

}

fn main() {
    let start = Path::new("/home/superuser/Documents/bu-from");
    let end = Path::new("/home/superuser/Documents/bu-to");

    Backup::run_through_directories(start, end);

    // Backup::copy(start, String::from(end));

}
