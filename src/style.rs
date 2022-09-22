//! Structs for handling the style file.

use anyhow::{anyhow, Result};
use printpdf::Mm;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseFloatError;
use std::path::PathBuf;
use std::str::FromStr;
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

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}mm, {}mm)", self.width.0, self.height.0)
    }
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

pub(crate) enum LineStyle {
    Solid,
    Dashed,
}

impl Display for LineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            LineStyle::Solid => write!(f, "solid"),
            LineStyle::Dashed => write!(f, "dashed"),
        }
    }
}

impl FromStr for LineStyle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "solid" => Ok(LineStyle::Solid),
            "dashed" => Ok(LineStyle::Dashed),
            _ => Err(anyhow!("Failed to convert to LineStyle from string")),
        }
    }
}

/// A box.
pub(crate) struct Box {
    pub(crate) position: Point,
    pub(crate) size: Size,
    pub(crate) line_width: Option<f32>,
    pub(crate) line_style: Option<LineStyle>,
}

impl Display for Box {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {}, {}, {})",
            self.position,
            self.size,
            self.line_width.unwrap_or(12.0),
            self.line_style.as_ref().unwrap_or(&LineStyle::Solid)
        )
    }
}

/// The postion & size of the `photo` in the YAML file.
pub(crate) struct Photo {
    pub(crate) position: Point,
    pub(crate) size: Size,
}

impl Display for Photo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.position, self.size,)
    }
}

/// A text box.
pub(crate) struct TextBox {
    pub(crate) position: Point,
    pub(crate) size: Size,
    pub(crate) value: String,
    pub(crate) font_size: Option<f32>,
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

fn parse_mm(raw_mm: &str) -> Result<Mm, ParseFloatError> {
    let mm_number = raw_mm.trim_end_matches("mm");
    let mm_as_float: f64 = mm_number.parse::<f64>()?;
    Ok(Mm(mm_as_float))
}

fn parse_option(name: &str, raw_option: &str) -> Result<f32, ParseFloatError> {
    let pattern = format!("{}=", name);
    let option_number = raw_option.trim_start_matches(&pattern);
    let option_value = option_number.parse::<f32>()?;
    Ok(option_value)
}

fn parse_line_style(raw_option: &str) -> Result<LineStyle> {
    let option_number = raw_option.trim_start_matches("line_style=");
    let option_value = option_number.parse::<LineStyle>()?;
    Ok(option_value)
}

fn parse_string(parameters: [&str; 4]) -> Result<Text> {
    let position = Point {
        x: parse_mm(parameters[0])?,
        y: parse_mm(parameters[1])?,
    };
    let text = Text {
        position,
        value: (*parameters[2]).to_owned(),
        font_size: parse_option("font_size", parameters[3])?,
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

fn parse_box(
    raw_pos_x: &str,
    raw_pos_y: &str,
    raw_width: &str,
    raw_height: &str,
    raw_line_options: Option<&&str>,
) -> Result<Box> {
    let position = Point {
        x: parse_mm(raw_pos_x)?,
        y: parse_mm(raw_pos_y)?,
    };
    let size = Size {
        width: parse_mm(raw_width)?,
        height: parse_mm(raw_height)?,
    };
    let mut line_width: Option<f32> = None;
    let mut line_style: Option<LineStyle> = None;
    if let Some(raw_option) = raw_line_options {
        if raw_option.starts_with("line_width") {
            line_width = Some(parse_option("line_width", raw_option)?);
        } else if raw_option.starts_with("line_style") {
            line_style = Some(parse_line_style(raw_option)?);
        }
    };
    Ok(Box {
        position,
        size,
        line_width,
        line_style,
    })
}

fn parse_photo(
    raw_pos_x: &str,
    raw_pos_y: &str,
    raw_width: &str,
    raw_height: &str,
) -> Result<Photo, ParseFloatError> {
    let position = Point {
        x: parse_mm(raw_pos_x)?,
        y: parse_mm(raw_pos_y)?,
    };
    let size = Size {
        width: parse_mm(raw_width)?,
        height: parse_mm(raw_height)?,
    };
    Ok(Photo { position, size })
}

pub(crate) enum Command {
    Text(Text),
    Line(Line),
    Box(Box),
    Photo(Photo),
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
                let raw_starting_x = split_line.get(1).expect("Missing x value for line!");
                let raw_starting_y = split_line.get(2).expect("Missing y value for line!");
                let raw_ending_x = split_line.get(3).expect("Missing x value for line!");
                let raw_ending_y = split_line.get(4).expect("Missing y value for line!");
                items.push(Command::Line(parse_line(
                    raw_starting_x,
                    raw_starting_y,
                    raw_ending_x,
                    raw_ending_y,
                )?));
            }
            Some(&"box") => {
                let raw_pos_x = split_line.get(1).expect("Missing x position for box!");
                let raw_pos_y = split_line.get(2).expect("Missing y position for box!");
                let raw_width = split_line.get(3).expect("Missing width for box!");
                let raw_height = split_line.get(4).expect("Missing height for box!");
                let raw_option = split_line.get(5);
                items.push(Command::Box(parse_box(
                    raw_pos_x, raw_pos_y, raw_width, raw_height, raw_option,
                )?));
            }
            Some(&"photo") => {
                let raw_pos_x = split_line.get(1).expect("Missing x position for photo!");
                let raw_pos_y = split_line.get(2).expect("Missing y position for photo!");
                let raw_width = split_line.get(3).expect("Missing width for box!");
                let raw_height = split_line.get(4).expect("Missing height for box!");
                items.push(Command::Photo(parse_photo(
                    raw_pos_x, raw_pos_y, raw_width, raw_height,
                )?));
            }
            _ => return Err(anyhow!("Unsupported command!")),
        }
    }
    Ok(items)
}
