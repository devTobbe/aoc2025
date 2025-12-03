// IO utility
use std::{
    fs::File,
    io::{self},
    path::Path,
};

pub fn read_file(file_path: &str) -> Result<String, io::Error> {
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File doesn't exist",
        ));
    }

    std::fs::read_to_string(file_path)
}

pub fn write_file(file_path: &str, contents: &str) -> Result<(), io::Error> {
    if !Path::new(file_path).exists() {
        let _ = File::create(file_path);
    }

    std::fs::write(file_path, contents)
}
