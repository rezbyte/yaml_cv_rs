//! Structs for handling the style file.

use anyhow::{anyhow, Result};
use printpdf::Mm;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Enumerate;
use std::num::ParseFloatError;
use std::path::PathBuf;
use std::vec::Vec;
pub(crate) mod command;
pub(crate) mod core;
use crate::style::command::{
    EducationExperience, History, Line, Lines, MiscBox, MultiLines, Photo, Text, TextBox, YMBox,
};
use crate::style::core::{FontOptions, LineOptions, LineStyle, Point, Size};

fn handle_missing<T>(
    expression: Option<T>,
    value_name: &str,
    command_name: &str,
    line_number: usize,
) -> T {
    let message = format!(
        "Missing {} value for {} at line: {}",
        value_name, command_name, line_number
    );
    expression.expect(&message)
}

fn parse_mm(raw_mm: &str) -> Result<Mm, ParseFloatError> {
    let mm_number = raw_mm.trim_end_matches("mm");
    let mm_as_float: f64 = mm_number.parse::<f64>()?;
    Ok(Mm(mm_as_float))
}

fn parse_option<T: std::str::FromStr>(name: &str, raw_option: &str) -> Result<T, T::Err> {
    let pattern = format!("{}=", name);
    let option_number = raw_option.trim_start_matches(&pattern);
    let option_value = option_number.parse::<T>()?;
    Ok(option_value)
}

fn parse_font_options(parameters: &[&str]) -> Result<FontOptions> {
    let mut font_size: Option<f64> = None;
    let mut font_face: Option<String> = None;
    for parameter in parameters {
        if let Some((command, value)) = (*parameter).to_owned().split_once('=') {
            match command {
                "font_size" => font_size = Some(value.parse::<f64>()?),
                "font_face" => font_face = Some(value.to_owned()),
                _ => continue,
            }
        }
    }
    Ok(FontOptions {
        font_size,
        font_face,
    })
}

fn parse_line_options(parameters: &[&str]) -> Result<LineOptions> {
    let mut line_width: Option<f32> = None;
    let mut line_style: Option<LineStyle> = None;
    for parameter in parameters {
        if let Some((command, value)) = (*parameter).to_owned().split_once('=') {
            match command {
                "line_width" => line_width = Some(value.parse::<f32>()?),
                "line_style" => line_style = Some(value.parse::<LineStyle>()?),
                _ => continue,
            }
        }
    }
    Ok(LineOptions {
        line_width,
        line_style,
    })
}

fn parse_string(parameters: &[&str], line_number: usize) -> Result<Text> {
    let raw_x = *handle_missing(parameters.get(1), "x", "string", line_number);
    let raw_y = *handle_missing(parameters.get(2), "y", "string", line_number);
    let raw_value = *handle_missing(parameters.get(3), "value", "string", line_number);
    let position = Point {
        x: parse_mm(raw_x)?,
        y: parse_mm(raw_y)?,
    };
    Ok(Text {
        position,
        value: raw_value.to_owned(),
        font_options: parse_font_options(parameters)?,
    })
}

fn parse_line(parameters: &[&str], line_number: usize) -> Result<Line> {
    let raw_starting_x = *handle_missing(parameters.get(1), "x1", "line", line_number);
    let raw_starting_y = *handle_missing(parameters.get(2), "2", "line", line_number);
    let raw_ending_x = *handle_missing(parameters.get(3), "x2", "line", line_number);
    let raw_ending_y = *handle_missing(parameters.get(4), "y2", "line", line_number);
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
        line_options: parse_line_options(parameters)?,
    })
}

fn parse_box(parameters: &[&str], line_number: usize) -> Result<command::Box> {
    let raw_pos_x = *handle_missing(parameters.get(1), "x", "box", line_number);
    let raw_pos_y = *handle_missing(parameters.get(2), "y", "box", line_number);
    let raw_width = *handle_missing(parameters.get(3), "width", "box", line_number);
    let raw_height = *handle_missing(parameters.get(4), "height", "box", line_number);
    let position = Point {
        x: parse_mm(raw_pos_x)?,
        y: parse_mm(raw_pos_y)?,
    };
    let size = Size {
        width: parse_mm(raw_width)?,
        height: parse_mm(raw_height)?,
    };
    Ok(command::Box {
        position,
        size,
        line_options: parse_line_options(parameters)?,
    })
}

fn parse_photo(parameters: &[&str], line_number: usize) -> Result<Photo, ParseFloatError> {
    let raw_pos_x = *handle_missing(parameters.get(1), "x", "photo", line_number);
    let raw_pos_y = *handle_missing(parameters.get(2), "y", "photo", line_number);
    let raw_width = *handle_missing(parameters.get(3), "width", "photo", line_number);
    let raw_height = *handle_missing(parameters.get(4), "height", "photo", line_number);
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

fn parse_textbox(parameters: &[&str], line_number: usize) -> Result<TextBox> {
    let raw_pos_x = *handle_missing(parameters.get(1), "x", "text box", line_number);
    let raw_pos_y = *handle_missing(parameters.get(2), "y", "text box", line_number);
    let raw_width = *handle_missing(parameters.get(3), "width", "text box", line_number);
    let raw_height = *handle_missing(parameters.get(4), "height", "text box", line_number);
    let raw_value = *handle_missing(parameters.get(5), "value", "text box", line_number);
    let position = Point {
        x: parse_mm(raw_pos_x)?,
        y: parse_mm(raw_pos_y)?,
    };
    let size = Size {
        width: parse_mm(raw_width)?,
        height: parse_mm(raw_height)?,
    };
    Ok(TextBox {
        position,
        size,
        value: raw_value.to_owned(),
        font_options: parse_font_options(parameters)?,
    })
}

fn parse_multilines(parameters: &[&str], line_number: usize) -> Result<MultiLines> {
    let raw_pos_x = *handle_missing(parameters.get(1), "x", "multi-lines", line_number);
    let raw_pos_y = *handle_missing(parameters.get(2), "y", "multi-lines", line_number);
    let raw_direction_x = *handle_missing(parameters.get(3), "dx", "multi-lines", line_number);
    let raw_direction_y = *handle_missing(parameters.get(4), "dy", "multi-lines", line_number);
    let raw_stroke_num = *handle_missing(
        parameters.get(5),
        "number of strokes",
        "multi-lines",
        line_number,
    );
    let raw_offset_x = *handle_missing(parameters.get(6), "sx", "multi-lines", line_number);
    let raw_offset_y = *handle_missing(parameters.get(7), "sy", "multi-lines", line_number);
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

fn parse_ymbox(parameters: &[&str], line_number: usize) -> Result<YMBox> {
    let raw_title = *handle_missing(parameters.get(1), "title", "ym box", line_number);
    let raw_height = *handle_missing(parameters.get(2), "height", "ym box", line_number);
    let raw_num = *handle_missing(parameters.get(3), "number", "ym box", line_number);
    let raw_value = *handle_missing(parameters.get(4), "value", "ym box", line_number);
    Ok(YMBox {
        title: raw_title.to_owned(),
        height: parse_mm(raw_height)?,
        num: raw_num.parse::<u32>()?,
        value: raw_value.to_owned(),
    })
}

fn parse_miscbox(parameters: &[&str], line_number: usize) -> Result<MiscBox> {
    let raw_title = *handle_missing(parameters.get(1), "title", "misc box", line_number);
    let raw_y = *handle_missing(parameters.get(2), "y", "misc box", line_number);
    let raw_height = *handle_missing(parameters.get(3), "height", "misc box", line_number);
    let raw_value = *handle_missing(parameters.get(4), "value", "misc box", line_number);
    Ok(MiscBox {
        title: raw_title.to_owned(),
        y: parse_mm(raw_y)?,
        height: parse_mm(raw_height)?,
        value: raw_value.to_owned(),
    })
}

fn parse_history(parameters: &[&str], line_number: usize) -> Result<History> {
    let raw_y = *handle_missing(parameters.get(1), "y", "history", line_number);
    let raw_year_x = *handle_missing(parameters.get(2), "year x", "history", line_number);
    let raw_month_x = *handle_missing(parameters.get(3), "month x", "history", line_number);
    let raw_value_x = *handle_missing(parameters.get(4), "value x", "history", line_number);
    let raw_padding = *handle_missing(parameters.get(5), "dy", "history", line_number);
    let raw_value = *handle_missing(parameters.get(6), "value", "history", line_number);

    Ok(History {
        y: parse_mm(raw_y)?,
        year_x: parse_mm(raw_year_x)?,
        month_x: parse_mm(raw_month_x)?,
        value_x: parse_mm(raw_value_x)?,
        padding: parse_mm(raw_padding)?,
        value: raw_value.to_owned(),
        font_options: parse_font_options(parameters)?,
    })
}

fn parse_education_experience(
    parameters: &[&str],
    line_number: usize,
) -> Result<EducationExperience> {
    let raw_y = *handle_missing(parameters.get(1), "y", "history", line_number);
    let raw_year_x = *handle_missing(parameters.get(2), "year x", "history", line_number);
    let raw_month_x = *handle_missing(parameters.get(3), "month x", "history", line_number);
    let raw_value_x = *handle_missing(parameters.get(4), "value x", "history", line_number);
    let raw_padding = *handle_missing(parameters.get(5), "dy", "history", line_number);
    let raw_caption_x = *handle_missing(parameters.get(6), "caption x", "history", line_number);
    let raw_ijo_x = *handle_missing(parameters.get(7), "ijo x", "history", line_number);

    Ok(EducationExperience {
        y: parse_mm(raw_y)?,
        year_x: parse_mm(raw_year_x)?,
        month_x: parse_mm(raw_month_x)?,
        value_x: parse_mm(raw_value_x)?,
        padding: parse_mm(raw_padding)?,
        caption_x: parse_mm(raw_caption_x)?,
        ijo_x: parse_mm(raw_ijo_x)?,
        font_options: parse_font_options(parameters)?,
    })
}

fn parse_lines(parameters: &[&str], line_number: usize) -> Result<Lines> {
    let raw_stroke_number = *handle_missing(parameters.get(1), "num", "misc box", line_number);

    let mut positions: Vec<Point> = Vec::new();
    let mut i = 2;
    while let [Some(raw_x), Some(raw_y)] = [parameters.get(i), parameters.get(i + 1)] {
        if raw_x.contains('=') || raw_y.contains('=') {
            break;
        }
        positions.push(Point {
            x: parse_mm(raw_x)?,
            y: parse_mm(raw_y)?,
        });
        i += 2;
    }

    let mut close: Option<bool> = None;
    if let Some(raw_option) = parameters.last() {
        close = Some(parse_option("close", raw_option)?);
    }

    Ok(Lines {
        stroke_number: raw_stroke_number.parse::<u32>()?,
        positions,
        line_options: parse_line_options(parameters)?,
        close,
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
    History(History),
    EducationExperience(EducationExperience),
    Lines(Lines),
}

type LineIterator = Enumerate<std::io::Lines<BufReader<File>>>;
fn get_lines(path: &PathBuf) -> std::io::Result<LineIterator> {
    let style_file = File::open(path)?;
    let reader = BufReader::new(style_file);
    Ok(reader.lines().enumerate())
}

pub(crate) fn read(path: &PathBuf) -> Result<Vec<Command>> {
    let mut items: Vec<Command> = Vec::new();
    for (index, line) in get_lines(path)? {
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
                let string = parse_string(&split_line, index)?;
                items.push(Command::Text(string));
            }
            Some(&"line") => {
                let line_command = parse_line(&split_line, index)?;
                items.push(Command::Line(line_command));
            }
            Some(&"box") => {
                let box_command = parse_box(&split_line, index)?;
                items.push(Command::Box(box_command));
            }
            Some(&"photo") => {
                let photo = parse_photo(&split_line, index)?;
                items.push(Command::Photo(photo));
            }
            Some(&"new_page") => {
                items.push(Command::NewPage);
            }
            Some(&"textbox") => {
                let textbox = parse_textbox(&split_line, index)?;
                items.push(Command::TextBox(textbox));
            }
            Some(&"multi_lines") => {
                let multi_lines = parse_multilines(&split_line, index)?;
                items.push(Command::MultiLines(multi_lines));
            }
            Some(&"ymbox") => {
                let ymbox = parse_ymbox(&split_line, index)?;
                items.push(Command::YMBox(ymbox));
            }
            Some(&"miscbox") => {
                let miscbox = parse_miscbox(&split_line, index)?;
                items.push(Command::MiscBox(miscbox));
            }
            Some(&"history") => {
                let history = parse_history(&split_line, index)?;
                items.push(Command::History(history));
            }
            Some(&"education_experience") => {
                let education_experience = parse_education_experience(&split_line, index)?;
                items.push(Command::EducationExperience(education_experience));
            }
            Some(&"lines") => {
                let lines = parse_lines(&split_line, index)?;
                items.push(Command::Lines(lines));
            }
            _ => {
                return Err(anyhow!(
                    "Unsupported command: {}!",
                    command_name.unwrap_or(&"")
                ))
            }
        }
    }
    Ok(items)
}
