//! Stores the basic structures that make up the commmand types.

use anyhow::{anyhow, Result};
use printpdf::Mm;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub(crate) const DEFAULT_FONT_FACE: &str = "mincho";
pub(crate) const DEFAULT_FONT_SIZE: f64 = 12.0_f64;
pub(crate) const DEFAULT_LINE_WIDTH: f32 = 0.5;

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

/// The patterns that can be used to draw lines.
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

// The options to customize the font.
pub(crate) struct FontOptions {
    pub(crate) font_size: Option<f64>,
    pub(crate) font_face: Option<String>,
}

impl Default for FontOptions {
    fn default() -> Self {
        FontOptions {
            font_size: Some(DEFAULT_FONT_SIZE),
            font_face: Some(DEFAULT_FONT_FACE.to_owned()),
        }
    }
}

impl Display for FontOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {})",
            self.font_size.unwrap_or(DEFAULT_FONT_SIZE),
            (&self.font_face)
                .clone()
                .unwrap_or_else(|| DEFAULT_FONT_FACE.to_owned()),
        )
    }
}

// The options to customize the line.
pub(crate) struct LineOptions {
    pub(crate) line_width: Option<f32>,
    pub(crate) line_style: Option<LineStyle>,
}

impl Default for LineOptions {
    fn default() -> Self {
        LineOptions {
            line_width: Some(DEFAULT_LINE_WIDTH),
            line_style: Some(LineStyle::Solid),
        }
    }
}

impl Display for LineOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {})",
            self.line_width.unwrap_or(DEFAULT_LINE_WIDTH),
            self.line_style.as_ref().unwrap_or(&LineStyle::Solid)
        )
    }
}
