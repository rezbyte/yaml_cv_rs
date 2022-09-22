//! Structs for handling the style file.

use anyhow::{anyhow, Result};
use printpdf::Mm;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseFloatError;
use std::path::PathBuf;
use std::vec::Vec;

// Represents a position in 2D space.
pub(crate) struct Point {
    pub(crate) x: Mm,
    pub(crate) y: Mm,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}mm, {}mm)", self.x.0, self.y.0)
    }
}

// Represents the size of a 2D object.
pub(crate) struct Size {
    pub(crate) width: Mm,
    pub(crate) height: Mm,
}

// Text.
pub(crate) struct Text {
    pub(crate) position: Point,
    pub(crate) value: String,
    pub(crate) font_size: f32,
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {}, {})", self.position, self.value, self.font_size)
    }
}

/// A line.
pub(crate) struct Line {
    pub(crate) start_position: Point,
    pub(crate) end_position: Point,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.start_position, self.end_position)
    }
}

/// A box.
pub(crate) struct Box {
    pub(crate) position: Point,
    pub(crate) size: Size,
}

/// The postion & size of the `photo` in the YAML file.
pub(crate) struct Photo {
    pub(crate) position: Point,
    pub(crate) size: Size,
}

/// A text box.
pub(crate) struct TextBox {
    pub(crate) position: Point,
    pub(crate) size: Size,
    pub(crate) value: String,
}

/// A set of procedurally generated lines.
pub(crate) struct MultiLines {
    pub(crate) start_position: Point,
    pub(crate) d_position: Point,
    pub(crate) stroke_number: u32,
    pub(crate) s_position: Point,
}

/// A time table.
pub(crate) struct YMBox {
    pub(crate) title: String,
    pub(crate) height: Mm,
    pub(crate) num: u32,
    pub(crate) value: String,
}

/// A text box with a title.
pub(crate) struct MiscBox {
    pub(crate) title: String,
    pub(crate) y: Mm,
    pub(crate) height: Mm,
    pub(crate) value: String,
}

pub(crate) enum Command {
    Text(Text),
    Line(Line),
}

fn parse_mm(raw_mm: &str) -> Result<Mm, ParseFloatError> {
    let mm_number = raw_mm.trim_end_matches("mm");
    let mm_as_float: f64 = mm_number.parse::<f64>()?;
    Ok(Mm(mm_as_float))
}

fn parse_font_size(raw_font_size: &str) -> Result<f32, ParseFloatError> {
    let font_size_number = raw_font_size.trim_start_matches("font_size=");
    let font_size = font_size_number.parse::<f32>()?;
    Ok(font_size)
}

fn parse_string(parameters: [&str; 4]) -> Result<Text> {
    let position = Point {
        x: parse_mm(parameters[0])?,
        y: parse_mm(parameters[1])?,
    };
    let text = Text {
        position,
        value: (*parameters[2]).to_owned(),
        font_size: parse_font_size(parameters[3])?,
    };
    Ok(text)
}

fn parse_line(
    raw_starting_x: &str,
    raw_starting_y: &str,
    raw_ending_x: &str,
    raw_ending_y: &str,
) -> Result<Line, ParseFloatError> {
    let start_position = Point {
        x: parse_mm(raw_starting_x)?,
        y: parse_mm(raw_starting_y)?,
    };
    let end_position = Point {
        x: parse_mm(raw_ending_x)?,
        y: parse_mm(raw_ending_y)?,
    };
    Ok(Line {
        start_position,
        end_position,
    })
}

pub(crate) fn read(path: PathBuf) -> Result<Vec<Command>> {
    let style_file = File::open(path)?;
    let reader = BufReader::new(style_file);
    let mut items: Vec<Command> = Vec::new();
    for (__index, line) in reader.lines().enumerate() {
        let line = line?;
        // Handle comments
        if line.starts_with('#') {
            continue;
        }
        let split_line: Vec<&str> = line.split(',').collect();
        let command_name = split_line.first();
        match command_name {
            Some(&"string") => {
                let raw_x: &str = split_line.get(1).expect("Missing x value for string!");
                let raw_y = split_line.get(2).expect("Missing y value for string!");
                let raw_value = split_line.get(3).expect("Missing value for string!");
                let raw_font_size = split_line
                    .get(4)
                    .expect("Missing font size value for string!");
                items.push(Command::Text(parse_string([
                    raw_x,
                    raw_y,
                    raw_value,
                    raw_font_size,
                ])?));
            }
            Some(&"line") => {
                let raw_starting_x = split_line.get(1).expect("Missing x value for string!");
                let raw_starting_y = split_line.get(2).expect("Missing y value for string!");
                let raw_ending_x = split_line.get(3).expect("Missing x value for string!");
                let raw_ending_y = split_line.get(4).expect("Missing y value for string!");
                items.push(Command::Line(parse_line(
                    raw_starting_x,
                    raw_starting_y,
                    raw_ending_x,
                    raw_ending_y,
                )?));
            }
            _ => return Err(anyhow!("Unsupported command!")),
        }
    }
    Ok(items)
}
