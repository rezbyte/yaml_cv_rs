//! Contains functions to get & process fonts.

use crate::style::core::DEFAULT_FONT_SIZE;
use anyhow::{anyhow, Result};
use printpdf::{IndirectFontRef, Mm, PdfDocumentReference, Pt};
use std::collections::HashMap;
use std::fs::File;

pub(crate) type FontMap<'a> = HashMap<&'a str, IndirectFontRef>;
#[allow(unused_results)]
pub(crate) fn get_fonts<'a>(doc: &PdfDocumentReference) -> Result<FontMap<'a>> {
    let mut fonts = HashMap::new();
    fonts.insert(
        "mincho",
        doc.add_external_font(File::open("fonts/ipaexm.ttf")?)?,
    );
    fonts.insert(
        "gothic",
        doc.add_external_font(File::open("fonts/ipaexg.ttf")?)?,
    );
    Ok(fonts)
}

pub(crate) fn handle_font<'a>(
    name: &'a String,
    fonts: &'a FontMap<'a>,
) -> Result<&'a IndirectFontRef> {
    if let Some(font) = fonts.get(name.as_str()) {
        Ok(font)
    } else {
        Err(anyhow!("Failed to fetch font: {}", name))
    }
}

pub(crate) fn font_size_to_mm(font_size: Option<f64>) -> Mm {
    let font_size = font_size.unwrap_or(DEFAULT_FONT_SIZE);
    Mm::from(Pt(font_size))
}
