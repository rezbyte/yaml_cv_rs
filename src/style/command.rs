//! The commands supported in the style file

use crate::style::core::{FontOptions, LineOptions, Point, Size};
use printpdf::Mm;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};

/// A string.
pub(crate) struct Text {
    pub(crate) position: Point,
    pub(crate) value: String,
    pub(crate) font_options: FontOptions,
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {}, {})",
            self.position, self.value, self.font_options
        )
    }
}

/// A line.
pub(crate) struct Line {
    pub(crate) start_position: Point,
    pub(crate) end_position: Point,
    pub(crate) line_options: LineOptions,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {}, {})",
            self.start_position, self.end_position, self.line_options
        )
    }
}

/// A box.
pub(crate) struct Box {
    pub(crate) position: Point,
    pub(crate) size: Size,
    pub(crate) line_options: LineOptions,
}

impl Display for Box {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {}, {})",
            self.position, self.size, self.line_options,
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
    pub(crate) font_options: FontOptions,
}

impl Display for TextBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {}, {}, {})",
            self.position, self.size, self.value, self.font_options,
        )
    }
}

/// A set of procedurally generated lines.
pub(crate) struct MultiLines {
    pub(crate) start_position: Point,
    pub(crate) direction: Point,
    pub(crate) stroke_number: u32,
    pub(crate) position_offset: Point,
}

impl Display for MultiLines {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {}, {}, {})",
            self.start_position, self.direction, self.stroke_number, self.position_offset,
        )
    }
}

/// A row for the time table.
pub(crate) struct YMBox {
    pub(crate) title: String,
    pub(crate) height: Mm,
    pub(crate) num: u32,
    pub(crate) value: String,
}

impl Display for YMBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {}, {}, {})",
            self.title, self.height.0, self.num, self.value,
        )
    }
}

/// A text box with a title.
pub(crate) struct MiscBox {
    pub(crate) title: String,
    pub(crate) y: Mm,
    pub(crate) height: Mm,
    pub(crate) value: String,
}

impl Display for MiscBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {}, {}, {})",
            self.title, self.y.0, self.height.0, self.value,
        )
    }
}

/// A time table.
pub(crate) struct History {
    pub(crate) y: Mm,
    pub(crate) year_x: Mm,
    pub(crate) month_x: Mm,
    pub(crate) value_x: Mm,
    pub(crate) padding: Mm,
    pub(crate) value: String,
    pub(crate) font_options: FontOptions,
}

impl Display for History {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "(({}, {}, {}, {}), {}, {}, {})",
            self.y.0,
            self.year_x.0,
            self.month_x.0,
            self.value_x.0,
            self.padding.0,
            self.value,
            self.font_options,
        )
    }
}

/// An employment & education history table.
pub(crate) struct EducationExperience {
    pub(crate) y: Mm,
    pub(crate) year_x: Mm,
    pub(crate) month_x: Mm,
    pub(crate) value_x: Mm,
    pub(crate) padding: Mm,
    pub(crate) caption_x: Mm,
    pub(crate) ijo_x: Mm,
    pub(crate) font_options: FontOptions,
}

impl Display for EducationExperience {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "(({}mm, {}mm, {}mm, {}mm), {}mm, ({}mm, {}mm), {})",
            self.y.0,
            self.year_x.0,
            self.month_x.0,
            self.value_x.0,
            self.padding.0,
            self.caption_x.0,
            self.ijo_x.0,
            self.font_options
        )
    }
}

/// A custom shape defined by a set of positions.
pub(crate) struct Lines {
    pub(crate) stroke_number: u32,
    pub(crate) positions: Vec<Point>,
    pub(crate) line_options: LineOptions,
    pub(crate) close: Option<bool>,
}

impl Display for Lines {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut position_text: String = String::new();
        for point in &self.positions {
            let point_text = format!("{}, ", point);
            position_text.push_str(&point_text);
        }
        write!(
            f,
            "({}, {}{}, {})",
            self.stroke_number,
            position_text,
            self.line_options,
            self.close.unwrap_or(false)
        )
    }
}
