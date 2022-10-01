//! Stores the basic structures that make up the commmand types.

use anyhow::{anyhow, Result};
use printpdf::Mm;
use printpdf::Point as PtPoint;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

pub(crate) const DEFAULT_FONT_FACE: &str = "mincho";
pub(crate) const DEFAULT_FONT_SIZE: f64 = 12.0_f64;
pub(crate) const DEFAULT_LINE_WIDTH: f32 = 0.5;

// Represents a position in 2D space.
#[derive(Copy, Clone, Default)]
pub(crate) struct Point {
    pub(crate) x: Mm,
    pub(crate) y: Mm,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}mm, {}mm)", self.x.0, self.y.0)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl From<PtPoint> for Point {
    fn from(ptpoint: PtPoint) -> Self {
        Self {
            x: Mm::from(ptpoint.x),
            y: Mm::from(ptpoint.y),
        }
    }
}

impl From<Point> for PtPoint {
    fn from(point: Point) -> Self {
        Self {
            x: point.x.into_pt(),
            y: point.y.into_pt(),
        }
    }
}

// Represents the size of a 2D object.
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Clone)]
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
#[derive(Copy, Clone)]
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
