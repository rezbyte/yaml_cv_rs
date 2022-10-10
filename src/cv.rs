//! Creates the CV in a PDF file.

use crate::style::command::{
    Box, EducationExperience, History, HistoryPosition, Line, Lines, MultiLines, Photo, Text,
    TextBox,
};
use crate::style::core::{
    FontOptions, LineOptions, LineStyle, Point, Size, DEFAULT_FONT_FACE, DEFAULT_FONT_SIZE,
};
use crate::style::Command;
use crate::yaml::{Entry, YAMLArgs};
use anyhow::Result;
use printpdf::image_crate::codecs::jpeg::JpegDecoder;
use printpdf::{
    Image, ImageTransform, LineDashPattern, Mm, PdfDocument, PdfDocumentReference,
    PdfLayerReference, Point as PtPoint,
};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use self::font::{font_size_to_mm, get_fonts, handle_font, FontMap};
use self::value::{handle_history_value, handle_value};
mod font;
mod value;

const MARGIN: Mm = Mm(12.7);
const MARGIN_AS_POINT: Point = Point {
    x: MARGIN,
    y: MARGIN,
};
const A4_WIDTH: f64 = 210.0_f64;
const A4_HEIGHT: f64 = 297.0_f64;
const DPI: f64 = 75.0_f64;

fn handle_line_options(options: &LineOptions, layer: &PdfLayerReference) {
    let width = options.line_width.unwrap_or_default();
    layer.set_outline_thickness(width.into());

    let line_style = options.line_style.unwrap_or_default();
    match line_style {
        LineStyle::Solid => layer.set_line_dash_pattern(LineDashPattern::default()),
        LineStyle::Dashed => {
            let pattern = LineDashPattern::new(0, Some(1), None, None, None, None, None);
            layer.set_line_dash_pattern(pattern);
        }
    }
}

fn draw_string(
    string: &Text,
    layer: &PdfLayerReference,
    fonts: &FontMap<'_>,
    inputs: &YAMLArgs,
) -> Result<()> {
    let font_size = string.font_options.font_size.unwrap_or(DEFAULT_FONT_SIZE);
    let value = handle_value(&string.value, inputs).unwrap_or(&string.value);
    let default_font = &DEFAULT_FONT_FACE.to_owned();
    let font = handle_font(
        string
            .font_options
            .font_face
            .as_ref()
            .unwrap_or(default_font),
        fonts,
    )?;
    let font_size_mm = font_size_to_mm(string.font_options.font_size);
    // Handle new lines in value
    let mut y_offset = Mm(0.0_f64);
    for line in value.split('\n') {
        layer.use_text(
            line,
            font_size,
            string.position.x + MARGIN,
            string.position.y + MARGIN - font_size_mm - y_offset,
            font,
        );
        y_offset += font_size_mm;
    }

    Ok(())
}

fn draw_line(line: &Line, layer: &PdfLayerReference) {
    let points: std::vec::Vec<(printpdf::Point, _)> = vec![
        ((line.start_position + MARGIN_AS_POINT).into(), false),
        (
            (line.start_position + line.end_position + MARGIN_AS_POINT).into(),
            false,
        ),
    ];

    handle_line_options(&line.line_options, layer);

    layer.add_shape(printpdf::Line {
        points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    });
}

fn draw_box(the_box: &Box, layer: &PdfLayerReference) {
    let points: std::vec::Vec<(printpdf::Point, _)> = vec![
        (
            PtPoint::new(
                the_box.position.x + the_box.size.width + MARGIN,
                the_box.position.y + MARGIN,
            ),
            false,
        ),
        (
            PtPoint::new(
                the_box.position.x + the_box.size.width + MARGIN,
                the_box.position.y + the_box.size.height + MARGIN,
            ),
            false,
        ),
        (
            PtPoint::new(
                the_box.position.x + MARGIN,
                the_box.position.y + the_box.size.height + MARGIN,
            ),
            false,
        ),
        (
            PtPoint::new(the_box.position.x + MARGIN, the_box.position.y + MARGIN),
            false,
        ),
    ];
    handle_line_options(&the_box.line_options, layer);
    layer.add_shape(printpdf::Line {
        points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    });
}

fn load_image(path: &Path) -> Result<Image> {
    let image_file = File::open(path)?;
    let image = Image::try_from(JpegDecoder::new(&image_file)?)?;
    Ok(image)
}

fn size_to_scale(size: Mm, position: Mm) -> f64 {
    let final_pos = position + size;
    final_pos.0 / position.0
}

fn draw_photo(photo: &Photo, image_path: &Path, layer: &PdfLayerReference) -> Result<()> {
    let image = load_image(image_path)?;
    let transform = ImageTransform {
        translate_x: Some(photo.position.x + Mm(11.0)),
        translate_y: Some(photo.position.y - Mm(28.0)),
        rotate: None,
        scale_x: Some(size_to_scale(photo.size.width, photo.position.x)),
        scale_y: Some(size_to_scale(photo.size.height, photo.position.y)),
        dpi: Some(115.0_f64),
    };
    image.add_to_layer(layer.clone(), transform);
    Ok(())
}

fn new_page(doc: &PdfDocumentReference) -> PdfLayerReference {
    let (new_page, new_layer) = doc.add_page(Mm(A4_WIDTH), Mm(A4_HEIGHT), "Layer 1");
    doc.get_page(new_page).get_layer(new_layer)
}

fn draw_textbox(
    textbox: &TextBox,
    layer: &PdfLayerReference,
    fonts: &FontMap<'_>,
    inputs: &YAMLArgs,
) -> Result<()> {
    let string = Text {
        position: textbox.position,
        value: handle_value(&textbox.value, inputs)?.to_string(),
        font_options: textbox.font_options.clone(),
    };
    draw_string(&string, layer, fonts, inputs)?;
    Ok(())
}

fn draw_multilines(multilines: &MultiLines, layer: &PdfLayerReference) {
    let mut pos = multilines.start_position;
    for __i in 0..multilines.stroke_number {
        let line = Line {
            start_position: pos,
            end_position: multilines.direction,
            line_options: LineOptions::default(),
        };
        draw_line(&line, layer);
        pos += multilines.position_offset;
    }
}

fn draw_lines(lines: &Lines, layer: &PdfLayerReference) -> Result<()> {
    let is_closed = lines.close.unwrap_or(true);
    let start_position: Point = *lines
        .positions
        .get(0)
        .expect("Failed to get first position in lines");
    let mut points: std::vec::Vec<(printpdf::Point, _)> =
        vec![((start_position + MARGIN_AS_POINT).into(), false)];
    let stroke_number: usize = usize::try_from(lines.stroke_number)?;
    for i in 1..stroke_number {
        let (previous_point, _) = *points.get(i - 1).expect("Failed to get previous value");
        let previous_value = Point::from(previous_point);
        let end_position: Point = *lines.positions.get(i).unwrap_or(&Point::default());
        points.push(((previous_value + end_position).into(), false));
    }
    handle_line_options(&lines.line_options, layer);
    layer.add_shape(printpdf::Line {
        points,
        is_closed,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    });
    Ok(())
}

fn draw_table(
    header: Option<&Text>,
    table: &[Entry],
    positions: &HistoryPosition,
    font_options: &FontOptions,
    layer: &PdfLayerReference,
    fonts: &FontMap<'_>,
    inputs: &YAMLArgs,
) -> Result<Mm> {
    let mut final_y = positions.y + positions.padding;
    if let Some(header_ref) = header {
        draw_string(header_ref, layer, fonts, inputs)?;
        final_y = header_ref.position.y - positions.padding;
    }
    let font_size_mm = font_size_to_mm(font_options.font_size);
    for entry in table.iter() {
        let year = Text {
            position: Point {
                x: positions.year_x,
                y: final_y,
            },
            value: entry.year.clone().unwrap_or_default(),
            font_options: font_options.clone(),
        };
        draw_string(&year, layer, fonts, inputs)?;
        let month_value: String = if let Some(month) = entry.month {
            month.to_string()
        } else {
            "".to_owned()
        };
        let month_offset = if month_value.len() > 1 {
            font_size_mm / 3.0_f64
        } else {
            Mm(0.0)
        };
        let month = Text {
            position: Point {
                x: positions.month_x - month_offset,
                y: final_y,
            },
            value: month_value,
            font_options: font_options.clone(),
        };
        draw_string(&month, layer, fonts, inputs)?;
        let value = Text {
            position: Point {
                x: positions.value_x,
                y: final_y,
            },
            value: entry.value.clone(),
            font_options: font_options.clone(),
        };
        draw_string(&value, layer, fonts, inputs)?;
        final_y -= positions.padding;
    }
    Ok(final_y)
}

#[allow(unused_results)]
fn draw_education_experience(
    education_experience: &EducationExperience,
    layer: &PdfLayerReference,
    fonts: &FontMap<'_>,
    inputs: &YAMLArgs,
) -> Result<()> {
    let education_header = Text {
        position: Point {
            x: education_experience.caption_x,
            y: education_experience.positions.y,
        },
        value: "学歴".to_owned(),
        font_options: education_experience.font_options.clone(),
    };
    let current_y = draw_table(
        Some(&education_header),
        &inputs.education,
        &education_experience.positions,
        &education_experience.font_options,
        layer,
        fonts,
        inputs,
    )?;
    let experience_header = Text {
        position: Point {
            x: education_experience.caption_x,
            y: current_y,
        },
        value: "職歴".to_owned(),
        font_options: education_experience.font_options.clone(),
    };
    draw_table(
        Some(&experience_header),
        &inputs.experience,
        &education_experience.positions,
        &education_experience.font_options,
        layer,
        fonts,
        inputs,
    )?;
    Ok(())
}

#[allow(unused_results)]
fn draw_history(
    history: &History,
    layer: &PdfLayerReference,
    fonts: &FontMap<'_>,
    inputs: &YAMLArgs,
) -> Result<()> {
    draw_table(
        None,
        handle_history_value(&history.value, inputs)?,
        &history.positions,
        &history.font_options,
        layer,
        fonts,
        inputs,
    )?;
    Ok(())
}

pub(crate) fn make(
    output_path: &Path,
    style_script: Vec<Command>,
    inputs: &YAMLArgs,
) -> Result<()> {
    let (doc, page1, layer1) = PdfDocument::new("CV", Mm(A4_WIDTH), Mm(A4_HEIGHT), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);
    let fonts = get_fonts(&doc)?;
    let image_path = Path::new("./photo.jpg");
    for command in style_script {
        match command {
            Command::Text(text) => draw_string(&text, &current_layer, &fonts, inputs)?,
            Command::Line(line) => draw_line(&line, &current_layer),
            Command::Box(the_box) => draw_box(&the_box, &current_layer),
            Command::Photo(photo) => draw_photo(&photo, image_path, &current_layer)?,
            Command::NewPage => current_layer = new_page(&doc),
            Command::TextBox(textbox) => draw_textbox(&textbox, &current_layer, &fonts, inputs)?,
            Command::MultiLines(multilines) => draw_multilines(&multilines, &current_layer),
            Command::YMBox(ymbox) => println!("The YM box '{}' was found!", ymbox),
            Command::MiscBox(miscbox) => println!("The misc box '{}' was found!", miscbox),
            Command::History(history) => draw_history(&history, &current_layer, &fonts, inputs)?,
            Command::EducationExperience(education_experience) => {
                draw_education_experience(&education_experience, &current_layer, &fonts, inputs)?;
            }
            Command::Lines(lines) => draw_lines(&lines, &current_layer)?,
        }
    }
    doc.save(&mut BufWriter::new(File::create(output_path)?))?;
    Ok(())
}
