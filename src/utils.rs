use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::path::Path;

use iced::Color;
use iced::window::Icon;
use iced::window::icon::from_rgba;
use image::error::DecodingError;
use image::{GenericImageView, ImageError, ImageResult};
use palette::{FromColor, Hsl, Srgb};
use regex::Regex;
use uuid::Uuid;

use crate::base_description_component::{DescriptionElement, parse_description_elements};

pub const FILE_VERSION: u32 = 2;

#[derive(Clone, Debug)]
pub struct DataEntry {
    pub id: Uuid,
    pub key: String,
    pub description: Vec<DescriptionElement>,
    pub description_raw: String,
}

impl DataEntry {
    pub fn new(key: &str, description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            key: key.to_string(),
            description: parse_description_elements(description.to_string()),
            description_raw: description.to_string(),
        }
    }
}

pub struct AppData<Message>
where
    Message: Clone,
{
    pub version: u32,
    pub entries: Vec<DataEntry>,
    _message: Message,
}

impl<Message> AppData<Message>
where
    Message: Clone,
{
    pub fn new(none: Message) -> AppData<Message> {
        AppData {
            version: FILE_VERSION,
            entries: Vec::new(),
            _message: none,
        }
    }

    pub fn load_file(&mut self, filename: String) -> io::Result<()> {
        let path = Path::new(&filename);

        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "File does not exist",
            ));
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let reg = Regex::new(r"(?P<key>(?:\\:|[^:])+):(?P<desc>.*)").unwrap();

        let first_line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "File is empty"))??;

        let version = first_line
            .trim()
            .parse::<u32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid version format"))?;

        if version > FILE_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Version mismatch",
            ));
        }

        let mut new_entries = Vec::new();

        for (index, line_result) in lines.enumerate() {
            let line = line_result?;
            if line.trim().is_empty() {
                continue;
            }

            if let Some(caps) = reg.captures(&line) {
                let key = caps["key"].replace(r"\:", ":");
                let desc = caps["desc"].replace(r"\:", ":");

                new_entries.push(DataEntry::new(key.trim(), desc.trim()));
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Malformed line at #{} (expected 'key:desc'): {}",
                        index + 2,
                        line
                    ),
                ));
            }
        }
        self.entries = new_entries;
        Ok(())
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
                        entry.description_raw.replace(":", r"\:")
                    )?;
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

pub fn load_icon() -> ImageResult<Icon> {
    let icon_bytes = include_bytes!("../assets/icon.png");

    let image = image::load_from_memory(icon_bytes);
    match image {
        Ok(image) => {
            let (width, height) = image.dimensions();
            let rgba = image.to_rgba8();

            match from_rgba(rgba.into_raw().into(), width, height) {
                Ok(icon) => Ok(icon),
                Err(error) => Err(ImageError::Decoding(DecodingError::new(
                    image::error::ImageFormatHint::Unknown,
                    error,
                ))),
            }
        }
        Err(error) => Err(error),
    }
}

pub fn shift_hue(color: Color, amount_degrees: f32) -> Color {
    let srgb = Srgb::new(color.r, color.g, color.b);
    let mut hsl = Hsl::from_color(srgb);
    hsl.hue = hsl.hue + amount_degrees;
    let shifted_rgb = Srgb::from_color(hsl);
    Color {
        r: shifted_rgb.red,
        g: shifted_rgb.green,
        b: shifted_rgb.blue,
        a: color.a,
    }
}
