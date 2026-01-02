use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Element {
    pub key: String,
    pub description: String,
}

pub struct AppData {
    pub version: i32,
    pub elements: Vec<Element>,
}

impl AppData {
    pub fn new() -> Result<AppData> {
        Ok(AppData {
            version: 1,
            elements: Vec::new(),
        })
    }

    pub fn load_file(filename: String) -> Result<AppData> {
        if !Path::exists(Path::new(&filename)) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "File does not exist",
            ));
        }

        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let first_line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "File is empty"))??;

        let version =
            first_line.trim().parse::<i32>().ok().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid version format")
            })?;

        let mut elements = Vec::new();
        while let Some(line_result) = lines.next() {
            let separator = line_result?.chars().next().unwrap();
            if separator == char::from_u32(0).unwrap() {
                break;
            } else if separator != char::from_u32(1).unwrap() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid element separator!",
                ));
            }

            let key = lines
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing Key"))??;
            let description = lines.next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::UnexpectedEof, "Missing Description")
            })??;

            elements.push(Element { key, description });
        }

        Ok(AppData { version, elements })
    }
}
