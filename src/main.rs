extern crate termion;
extern crate walkdir;

use walkdir::WalkDir;

use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub struct Backup;
impl Backup {

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

    pub fn run_through_directories(start: &Path, end: &Path) {
        let start_name: &str = start.to_str().unwrap();
        let end_name: &str = end.to_str().unwrap();

        for entry in WalkDir::new(start_name).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

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

    Backup::run_through_directories(start, end);

    // Backup::copy(start, String::from(end));

}
