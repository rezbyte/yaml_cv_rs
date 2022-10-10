//! Extract variables from values in Commands.

use crate::yaml::{Entry, YAMLArgs};
use anyhow::{anyhow, Result};

pub(crate) fn handle_value<'a>(value: &'a String, inputs: &'a YAMLArgs) -> Result<&'a String> {
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
            "$hobby" => Ok(&inputs.hobby),
            "$motivation" => Ok(&inputs.motivation),
            "$request" => Ok(&inputs.request),
            "$degree" => Ok(&inputs.degree),
            "$degree_year" => Ok(&inputs.degree_year),
            "$degree_affiliation" => Ok(&inputs.degree_affiliation),
            "$thesis_title" => Ok(&inputs.thesis_title),
            _ => Err(anyhow!("Unknown variable: {}", value)),
        }
    } else {
        Ok(value)
    }
}

pub(crate) fn handle_history_value<'a>(
    value: &'a String,
    inputs: &'a YAMLArgs,
) -> Result<&'a Vec<Entry>> {
    match value.as_str() {
        "$awards" => Ok(&inputs.awards),
        "$education" => Ok(&inputs.education),
        "$experience" => Ok(&inputs.experience),
        "$licences" => Ok(&inputs.licences),
        _ => Err(anyhow!("Unkown value: {}", value)),
    }
}
