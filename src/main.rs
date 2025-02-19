//! Kiew is a Command Line Interface, for scraping website.
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::redundant_pub_crate,
    clippy::semicolon_if_nothing_returned,
    clippy::alloc_instead_of_core,
    clippy::unwrap_in_result,
    clippy::todo,
    clippy::unreachable,
    clippy::mod_module_files,
    clippy::missing_panics_doc,
    clippy::dbg_macro,
    clippy::integer_division,
    clippy::string_slice,
    clippy::missing_assert_message
)]

use kiew::cmds::handles::handles_commands;

#[doc = "Kiew"]
#[tokio::main]
async fn main() {
    handles_commands().await;
}
