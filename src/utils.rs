use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::path::Path;

use crate::window_component::WindowContent;
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
                .add_window(WindowContent {
                    window_type: crate::window_component::WindowType::Warning,
                    title: "Version Mismatch".to_string(),
                    content: format!(
                        "Expected version {}, but found version {}.\nSome features may not work correctly.",
                        FILE_VERSION, version
                    ).to_string(),
                    window_width: None,
                });
        }

        let mut elements = Vec::new();
        while let Some(line_result) = lines.next() {
            match line_result {
                Err(e) => return Err(e),
                Ok(line) => {
                    if let Some((key, desc)) = line.split_once(':') {
                        elements.push(DataEntry {
                            key: key.trim().to_string(),
                            description: desc.trim().to_string(),
                        });
                        continue;
                    } else {
                        WindowManager::global()
                            .lock()
                            .unwrap()
                            .add_window(WindowContent {
                                window_type: crate::window_component::WindowType::Warning,
                                title: "Undifined line format.".to_string(),
                                content: format!(
                                    "Expected '<key>:<description>'  found {}.\nLine ignored.",
                                    line
                                )
                                .to_string(),
                                window_width: None,
                            });
                    }
                }
            }
        }

        Ok(AppData {
            version,
            entries: elements,
        })
    }
}
