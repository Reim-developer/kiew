//! Define all crates
#![deny(
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
    clippy::missing_assert_message,
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::shadow_unrelated,
    clippy::shadow_reuse,
    clippy::panic,
    clippy::wildcard_imports,
    clippy::cargo_common_metadata,
    clippy::similar_names,
    clippy::manual_assert,
    clippy::mut_mut,
    clippy::missing_const_for_fn,
    clippy::unused_self,
    clippy::redundant_else,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::large_stack_arrays,
    clippy::manual_map,
    clippy::default_trait_access,
    clippy::redundant_closure,
    clippy::match_bool,
    clippy::option_if_let_else,
    clippy::redundant_pattern_matching,
    clippy::new_without_default,
    clippy::mutex_atomic,
    clippy::if_same_then_else,
    clippy::unwrap_or_default,
    clippy::fn_params_excessive_bools,
    clippy::needless_pass_by_value,
    clippy::trivial_regex,
    clippy::implicit_clone,
    clippy::manual_let_else,
    clippy::redundant_type_annotations
)]

/// Core, is logic of Command Line application.
pub mod core {
    /// To match a element of website
    pub mod element;

    /// To find element of website and show
    /// all infomation, if this exist
    pub mod find_element;
}

/// CLI module, to define all commands of CLI
mod cli;

/// For handles all commands of CLI
pub mod cmds {
    /// Handles all commands in this module
    pub mod handles;
}

/// Define Colors/LogLevel
mod colors;
/// Define all error types
mod errors;
