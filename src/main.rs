//! A rust port of kaityo256's ``yaml_cv``

#![warn(
    elided_lifetimes_in_paths,
    keyword_idents,
    macro_use_extern_crate,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    unreachable_pub,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_results,
    clippy::pedantic,
    clippy::negative_feature_names,
    clippy::redundant_feature_names,
    clippy::wildcard_dependencies,
    clippy::allow_attributes_without_reason,
    clippy::as_conversions,
    clippy::as_underscore,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::default_union_representation,
    clippy::empty_structs_with_brackets,
    clippy::filetype_is_file,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::map_err_ignore,
    clippy::mixed_read_write_in_expression,
    clippy::mod_module_files,
    clippy::multiple_inherent_impl,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::todo,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::cognitive_complexity,
    clippy::suboptimal_flops
)]
#![deny(
    missing_abi,
    pointer_structural_match,
    unsafe_op_in_unsafe_fn,
    clippy::default_numeric_fallback,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::string_slice,
    clippy::debug_assert_with_mut_call
)]
#![forbid(unsafe_code)]

use anyhow::Result;
use clap::Parser;
use serde_yaml::from_str;
use std::fs::read_to_string;
use style::Command;

mod args;
mod style;
mod yaml;

fn main() -> Result<()> {
    let cli = args::Args::parse();

    let raw_input_file = read_to_string(cli.input)?;
    let input_file: yaml::YAMLArgs = from_str(&raw_input_file)?;

    let style_file = style::read(cli.style)?;

    println!("Hello, {}!", input_file.name_kana);

    for command in style_file {
        match command {
            Command::Text(text) => {
                println!("The string '{}' was found!", text.value);
            }
            Command::Line(line) => {
                println!("The line '{}' was found!", line);
            }
            _ => {}
        }
    }

    Ok(())
}
