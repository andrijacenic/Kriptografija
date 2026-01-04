use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::path::Path;

use regex::Regex;

use crate::window_component::{WindowContent, WindowType};
use crate::window_manager::WindowManager;

pub const FILE_VERSION: u32 = 1;

#[derive(Debug, Clone)]
pub struct DataEntry {
    pub key: String,
    pub description: String,
}

pub struct AppData {
    pub version: u32,
    pub entries: Vec<DataEntry>,
}

impl AppData {
    pub fn new() -> Result<AppData> {
        Ok(AppData {
            version: 1,
            entries: Vec::new(),
        })
    }

    pub fn load_file(filename: String) -> Result<AppData> {
        let reg = Regex::new(r"(?P<key>(?:\\:|[^:])+):(?P<desc>.*)").unwrap();

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
            first_line.trim().parse::<u32>().ok().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid version format")
            })?;

        if version != FILE_VERSION {
            WindowManager::global()
                .lock()
                .unwrap()
                .add_window(WindowContent::new(
                    WindowType::Warning,
                    "Version Mismatch".to_string(),
                    format!(
                        "Expected version {}, but found version {}.\nSome features may not work correctly.",
                        FILE_VERSION, version
                    ).to_string(),
                    None,
                    false,
                    true,
                ));
        }

        let mut elements = Vec::new();
        while let Some(line_result) = lines.next() {
            match line_result {
                Err(e) => return Err(e),
                Ok(line) => {
                    if let Some((key, desc)) = reg.captures(&line).map(|cap| {
                        (
                            cap.name("key").unwrap().as_str().replace(r"\:", ":"),
                            cap.name("desc").unwrap().as_str().replace(r"\:", ":"),
                        )
                    }) {
                        elements.push(DataEntry {
                            key: key.trim().to_string(),
                            description: desc.trim().to_string(),
                        });
                        print!("Loaded entry: key='{}', description='{}'\n", key, desc);
                    } else {
                        WindowManager::global()
                            .lock()
                            .unwrap()
                            .add_window(WindowContent::new(
                                WindowType::Warning,
                                "Undifined line format.".to_string(),
                                format!(
                                    "Expected '<key>:<description>'  found {}.\nLine ignored.",
                                    line
                                )
                                .to_string(),
                                None,
                                false,
                                true,
                            ));
                    }
                }
            }
        }

        Ok(AppData {
            version,
            entries: elements,
        })
    }

    pub fn save_file(&self, filename: String) -> Result<()> {
        match File::create(filename) {
            Ok(mut file) => {
                use std::io::Write;
                writeln!(file, "{}", self.version)?;

                for entry in &self.entries {
                    writeln!(
                        file,
                        "{}:{}",
                        entry.key.replace(":", r"\:"),
                        entry.description.replace(":", r"\:")
                    )?;
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
