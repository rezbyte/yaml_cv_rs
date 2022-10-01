//! Structs for handling the input YAML file,

use serde::Deserialize;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Deserialize)]
/// An entry in a table (such as the education table)
pub(crate) struct Entry {
    pub(crate) year: Option<String>,
    pub(crate) month: Option<u8>,
    pub(crate) value: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
/// The valid fields in the input YAML file.
pub(crate) struct YAMLArgs {
    pub(crate) date: String,
    pub(crate) name_kana: String,
    pub(crate) name: String,
    pub(crate) birth_day: String,
    pub(crate) gender: String,
    pub(crate) cell_phone: String,
    pub(crate) email: String,
    pub(crate) photo: PathBuf,
    pub(crate) address_kana: String,
    pub(crate) address: String,
    pub(crate) address_zip: String,
    pub(crate) tel: String,
    pub(crate) fax: String,
    pub(crate) address_kana2: String,
    pub(crate) address2: String,
    pub(crate) address_zip2: String,
    pub(crate) tel2: String,
    pub(crate) fax2: String,
    pub(crate) degree: String,
    pub(crate) degree_year: String,
    pub(crate) degree_affiliation: String,
    pub(crate) thesis_title: String,
    pub(crate) education: Vec<Entry>,
    pub(crate) experience: Vec<Entry>,
    pub(crate) licences: Vec<Entry>,
    pub(crate) awards: Vec<Entry>,
    pub(crate) teaching: String,
    pub(crate) affiliated_society: String,
    pub(crate) notices: String,
    pub(crate) commuting_time: String,
    pub(crate) dependents: String,
    pub(crate) spouse: String,
    pub(crate) supporting_spouse: String,
    pub(crate) hobby: String,
    pub(crate) motivation: String,
    pub(crate) request: String,
}
