//! Creates the CV in a PDF file.

use crate::style::command::{Box, Line, Lines, MultiLines, Photo, Text, TextBox};
use crate::style::core::{LineOptions, LineStyle, Point, DEFAULT_FONT_FACE, DEFAULT_FONT_SIZE};
use crate::style::Command;
use crate::yaml::YAMLArgs;
use anyhow::{anyhow, Result};
use printpdf::image_crate::codecs::jpeg::JpegDecoder;
use printpdf::{
    Image, ImageTransform, IndirectFontRef, LineDashPattern, Mm, PdfDocument, PdfDocumentReference,
    PdfLayerReference, Point as PtPoint, Pt,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const MARGIN: Mm = Mm(12.7);
const MARGIN_AS_POINT: Point = Point {
    x: MARGIN,
    y: MARGIN,
};
const A4_WIDTH: f64 = 210.0_f64;
const A4_HEIGHT: f64 = 297.0_f64;
const DPI: f64 = 75.0_f64;

type FontMap<'a> = HashMap<&'a str, IndirectFontRef>;
#[allow(unused_results)]
fn get_fonts<'a>(doc: &PdfDocumentReference) -> Result<FontMap<'a>> {
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

fn handle_font<'a>(name: &'a String, fonts: &'a FontMap<'a>) -> Result<&'a IndirectFontRef> {
    if let Some(font) = fonts.get(name.as_str()) {
        Ok(font)
    } else {
        Err(anyhow!("Failed to fetch font: {}", name))
    }
}

fn handle_value<'a>(value: &'a String, inputs: &'a YAMLArgs) -> Result<&'a String> {
    if value.starts_with('$') {
        match value.as_str() {
            "$date" => Ok(&inputs.date),
            "$name_kana" => Ok(&inputs.name_kana),
            "$name" => Ok(&inputs.name),
            "$birth_day" => Ok(&inputs.birth_day),
            "$gender" => Ok(&inputs.gender),
            "$cell_phone" => Ok(&inputs.cell_phone),
            "$email" => Ok(&inputs.email),
            "$address_kana" => Ok(&inputs.address_kana),
            "$address" => Ok(&inputs.address),
            "$address_zip" => Ok(&inputs.address_zip),
            "$tel" => Ok(&inputs.tel),
            "$fax" => Ok(&inputs.fax),
            "$address_kana2" => Ok(&inputs.address_kana2),
            "$address2" => Ok(&inputs.address2),
            "$address_zip2" => Ok(&inputs.address_zip2),
            "$tel2" => Ok(&inputs.tel2),
            "$fax2" => Ok(&inputs.fax2),
            "$commuting_time" => Ok(&inputs.commuting_time),
            "$dependents" => Ok(&inputs.dependents),
            "$spouse" => Ok(&inputs.spouse),
            "$supporting_spouse" => Ok(&inputs.supporting_spouse),
            _ => Err(anyhow!("Unknown variable: {}", value)),
        }
    } else {
        Ok(value)
    }
}

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
    layer.use_text(
        value,
        font_size,
        string.position.x + MARGIN,
        string.position.y + Mm::from(Pt(font_size)) + Mm(7.0),
        font,
    );
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

fn draw_photo(photo: &Photo, image_path: &Path, layer: &PdfLayerReference) -> Result<()> {
    let image = load_image(image_path)?;
    let transform = ImageTransform {
        translate_x: Some(photo.position.x + MARGIN),
        translate_y: Some(photo.position.y + MARGIN),
        rotate: None,
        scale_x: Some(photo.size.width.0),
        scale_y: Some(photo.size.height.0),
        dpi: Some(DPI),
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
    // Position has origin at top left of the box, need to convert it to bottom left
    let position_from_bottom_left = Point {
        x: textbox.position.x,
        y: textbox.position.y - textbox.size.height,
    };

    let bounding_box = Box {
        position: position_from_bottom_left,
        size: textbox.size,
        line_options: LineOptions::default(),
    };

    let center_position = Point {
        x: bounding_box.position.x + (bounding_box.size.width * 0.5_f64),
        y: bounding_box.position.y + (bounding_box.size.height * 0.5_f64),
    };
    let string = Text {
        position: center_position,
        value: textbox.value.clone(),
        font_options: textbox.font_options.clone(),
    };

    draw_box(&bounding_box, layer);
    draw_string(&string, layer, fonts, inputs)?;
    Ok(())
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
            Command::History(history) => println!("The history '{}' was found!", history),
            Command::EducationExperience(education_experience) => println!(
                "The education experience '{}' was found!",
                education_experience
            ),
            Command::Lines(lines) => draw_lines(&lines, &current_layer)?,
        }
    }
    doc.save(&mut BufWriter::new(File::create(output_path)?))?;
    Ok(())
}
