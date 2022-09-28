//! Creates the CV in a PDF file.

use crate::style::command::{Box, Line, MultiLines, Photo, Text, TextBox};
use crate::style::core::{LineOptions, Point, DEFAULT_FONT_SIZE};
use crate::style::Command;
use crate::yaml::YAMLArgs;
use anyhow::Result;
use printpdf::image_crate::codecs::jpeg::JpegDecoder;
use printpdf::Point as PtPoint;
use printpdf::{Image, ImageTransform, IndirectFontRef, Mm, PdfDocument, PdfLayerReference};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const A4_WIDTH: f64 = 210.0_f64;
const A4_HEIGHT: f64 = 297.0_f64;
const DPI: f64 = 75.0_f64;

fn draw_string(string: Text, layer: &PdfLayerReference, font: &IndirectFontRef) {
    layer.use_text(
        string.value,
        string.font_options.font_size.unwrap_or(DEFAULT_FONT_SIZE),
        string.position.x,
        string.position.y,
        font,
    );
}

fn draw_line(line: &Line, layer: &PdfLayerReference) {
    let points: std::vec::Vec<(printpdf::Point, _)> = vec![
        (line.start_position.into(), false),
        (line.end_position.into(), false),
    ];
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
            PtPoint::new(the_box.position.x + the_box.size.width, the_box.position.y),
            false,
        ),
        (
            PtPoint::new(
                the_box.position.x + the_box.size.width,
                the_box.position.y + the_box.size.height,
            ),
            false,
        ),
        (
            PtPoint::new(the_box.position.x, the_box.position.y + the_box.size.height),
            false,
        ),
        (PtPoint::new(the_box.position.x, the_box.position.y), false),
    ];
    layer.add_shape(printpdf::Line {
        points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    });
}

fn draw_photo(photo: &Photo, image: Image, layer: &PdfLayerReference) {
    let transform = ImageTransform {
        translate_x: Some(photo.position.x),
        translate_y: Some(photo.position.y),
        rotate: None,
        scale_x: Some(photo.size.width.0),
        scale_y: Some(photo.size.height.0),
        dpi: Some(DPI),
    };
    image.add_to_layer(layer.clone(), transform);
}

fn load_image(path: &Path) -> Result<Image> {
    let image_file = File::open(path)?;
    let image = Image::try_from(JpegDecoder::new(&image_file)?)?;
    Ok(image)
}

fn draw_textbox(textbox: &TextBox, layer: &PdfLayerReference, font: &IndirectFontRef) {
    let bounding_box = Box {
        position: textbox.position,
        size: textbox.size,
        line_options: LineOptions::default(),
    };

    let center_position = Point {
        x: textbox.position.x + (textbox.size.width * 0.5_f64),
        y: textbox.position.y + (textbox.size.height * 0.5_f64),
    };
    let string = Text {
        position: center_position,
        value: textbox.value.clone(),
        font_options: textbox.font_options.clone(),
    };

    draw_box(&bounding_box, layer);
    draw_string(string, layer, font);
}

fn draw_multilines(multilines: &MultiLines, layer: &PdfLayerReference) {
    let mut pos = multilines.start_position;
    for __i in 0..multilines.stroke_number {
        let line = Line {
            start_position: pos,
            end_position: pos + multilines.direction,
            line_options: LineOptions::default(),
        };
        draw_line(&line, layer);
        pos += multilines.position_offset;
    }
}

pub(crate) fn make(
    output_path: &Path,
    style_script: Vec<Command>,
    __inputs: &YAMLArgs,
) -> Result<()> {
    let (doc, page1, layer1) = PdfDocument::new("CV", Mm(A4_WIDTH), Mm(A4_HEIGHT), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(File::open("fonts/ipaexg.ttf")?)?;
    let image_path = Path::new("./photo.jpg");
    for command in style_script {
        match command {
            Command::Text(text) => {
                draw_string(text, &current_layer, &font);
            }
            Command::Line(line) => {
                draw_line(&line, &current_layer);
            }
            Command::Box(the_box) => {
                draw_box(&the_box, &current_layer);
            }
            Command::Photo(photo) => {
                let image = load_image(image_path)?;
                draw_photo(&photo, image, &current_layer);
            }
            Command::NewPage => {
                let (new_page, new_layer) = doc.add_page(Mm(A4_WIDTH), Mm(A4_HEIGHT), "Layer 1");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
            }
            Command::TextBox(textbox) => {
                draw_textbox(&textbox, &current_layer, &font);
            }
            Command::MultiLines(multilines) => {
                draw_multilines(&multilines, &current_layer);
            }
            Command::YMBox(ymbox) => {
                println!("The YM box '{}' was found!", ymbox);
            }
            Command::MiscBox(miscbox) => {
                println!("The misc box '{}' was found!", miscbox);
            }
            Command::History(history) => {
                println!("The history '{}' was found!", history);
            }
            Command::EducationExperience(education_experience) => {
                println!(
                    "The education experience '{}' was found!",
                    education_experience
                );
            }
            Command::Lines(lines) => {
                println!("The lines '{}' was found!", lines);
            }
        }
    }
    doc.save(&mut BufWriter::new(File::create(output_path)?))?;
    Ok(())
}
