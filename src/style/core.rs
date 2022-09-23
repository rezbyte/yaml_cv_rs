//! Stores the basic structures that make up the commmand types.

use anyhow::{anyhow, Result};
use printpdf::Mm;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
