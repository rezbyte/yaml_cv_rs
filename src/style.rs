//! Structs for handling the style file.

use anyhow::{anyhow, Result};
use printpdf::Mm;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseFloatError;
use std::path::PathBuf;
use std::vec::Vec;
mod command;
mod core;
use crate::style::command::{Line, MiscBox, MultiLines, Photo, Text, TextBox, YMBox};
use crate::style::core::{LineStyle, Point, Size};

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
) -> Result<command::Box> {
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
    Ok(command::Box {
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

fn parse_textbox(
    raw_pos_x: &str,
    raw_pos_y: &str,
    raw_width: &str,
    raw_height: &str,
    raw_value: &str,
    raw_font_size: Option<&&str>,
) -> Result<TextBox> {
    let position = Point {
        x: parse_mm(raw_pos_x)?,
        y: parse_mm(raw_pos_y)?,
    };
    let size = Size {
        width: parse_mm(raw_width)?,
        height: parse_mm(raw_height)?,
    };
    let mut font_size: Option<f32> = None;
    if let Some(raw_option) = raw_font_size {
        font_size = Some(parse_option("font_size", raw_option)?);
    };
    Ok(TextBox {
        position,
        size,
        value: raw_value.to_owned(),
        font_size,
    })
}

fn parse_multilines(
    raw_pos_x: &str,
    raw_pos_y: &str,
    raw_direction_x: &str,
    raw_direction_y: &str,
    raw_stroke_num: &str,
    raw_offset_x: &str,
    raw_offset_y: &str,
) -> Result<MultiLines> {
    let start_position = Point {
        x: parse_mm(raw_pos_x)?,
        y: parse_mm(raw_pos_y)?,
    };
    let d_position = Point {
        x: parse_mm(raw_direction_x)?,
        y: parse_mm(raw_direction_y)?,
    };
    let stroke_number: u32 = raw_stroke_num.parse::<u32>()?;
    let s_position = Point {
        x: parse_mm(raw_offset_x)?,
        y: parse_mm(raw_offset_y)?,
    };
    Ok(MultiLines {
        start_position,
        direction: d_position,
        stroke_number,
        position_offset: s_position,
    })
}

fn parse_ymbox(raw_title: &str, raw_height: &str, raw_num: &str, raw_value: &str) -> Result<YMBox> {
    Ok(YMBox {
        title: raw_title.to_owned(),
        height: parse_mm(raw_height)?,
        num: raw_num.parse::<u32>()?,
        value: raw_value.to_owned(),
    })
}

fn parse_miscbox(
    raw_title: &str,
    raw_y: &str,
    raw_height: &str,
    raw_value: &str,
) -> Result<MiscBox> {
    Ok(MiscBox {
        title: raw_title.to_owned(),
        y: parse_mm(raw_y)?,
        height: parse_mm(raw_height)?,
        value: raw_value.to_owned(),
    })
}

pub(crate) enum Command {
    Text(Text),
    Line(Line),
    Box(command::Box),
    Photo(Photo),
    NewPage,
    TextBox(TextBox),
    MultiLines(MultiLines),
    YMBox(YMBox),
    MiscBox(MiscBox),
}

#[allow(clippy::too_many_lines)]
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
        // Skip blank lines
        if line.is_empty() {
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
            Some(&"new_page") => {
                items.push(Command::NewPage);
            }
            Some(&"textbox") => {
                let raw_pos_x = split_line.get(1).expect("Missing x position for text box!");
                let raw_pos_y = split_line.get(2).expect("Missing y position for text box!");
                let raw_width = split_line.get(3).expect("Missing width for text box!");
                let raw_height = split_line.get(4).expect("Missing height for text box!");
                let raw_value = split_line.get(5).expect("Missing value for text box!");
                let raw_option = split_line.get(6);
                items.push(Command::TextBox(parse_textbox(
                    raw_pos_x, raw_pos_y, raw_width, raw_height, raw_value, raw_option,
                )?));
            }
            Some(&"multi_lines") => {
                let raw_pos_x = split_line
                    .get(1)
                    .expect("Missing x position for multi-lines!");
                let raw_pos_y = split_line
                    .get(2)
                    .expect("Missing y position for multi-lines!");
                let raw_direction_x = split_line
                    .get(3)
                    .expect("Missing dx position for multi-lines!");
                let raw_direction_y = split_line
                    .get(4)
                    .expect("Missing dy position for multi-lines!");
                let raw_stroke_num = split_line
                    .get(5)
                    .expect("Missing number of strokes for multi-lines!");
                let raw_offset_x = split_line
                    .get(6)
                    .expect("Missing sx position for multi-lines!");
                let raw_offset_y = split_line
                    .get(7)
                    .expect("Missing sy position for multi-lines!");
                items.push(Command::MultiLines(parse_multilines(
                    raw_pos_x,
                    raw_pos_y,
                    raw_direction_x,
                    raw_direction_y,
                    raw_stroke_num,
                    raw_offset_x,
                    raw_offset_y,
                )?));
            }
            Some(&"ymbox") => {
                let raw_title = split_line.get(1).expect("Missing title for ym box!");
                let raw_height = split_line.get(2).expect("Missing height for ym box!");
                let raw_num = split_line.get(3).expect("Missing number for ym box!");
                let raw_value = split_line.get(4).expect("Missing height for ym box!");
                items.push(Command::YMBox(parse_ymbox(
                    raw_title, raw_height, raw_num, raw_value,
                )?));
            }
            Some(&"miscbox") => {
                let raw_title = split_line.get(1).expect("Missing title for misc box!");
                let raw_y = split_line.get(2).expect("Missing y value for misc box!");
                let raw_height = split_line.get(3).expect("Missing height for misc box!");
                let raw_value = split_line.get(4).expect("Missing value for misc box!");
                items.push(Command::MiscBox(parse_miscbox(
                    raw_title, raw_y, raw_height, raw_value,
                )?));
            }
            _ => {
                return Err(anyhow!(
                    "Unsupported command {}!",
                    command_name.unwrap_or(&"")
                ))
            }
        }
    }
    Ok(items)
}
