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
    clippy::redundant_type_annotations,
    clippy::absolute_paths,
    clippy::unnecessary_lazy_evaluations,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_allocation,
    clippy::manual_strip,
    clippy::multiple_inherent_impl,
    clippy::empty_structs_with_brackets,
    clippy::empty_enum,
    clippy::redundant_field_names,
    clippy::redundant_static_lifetimes,
    clippy::unnecessary_to_owned,
    clippy::inline_always,
    clippy::large_futures,
    clippy::vec_box,
    clippy::manual_memcpy,
    clippy::slow_vector_initialization,
    clippy::suboptimal_flops,
    clippy::borrow_as_ptr,
    clippy::missing_safety_doc,
    clippy::mutex_integer,
    clippy::ptr_as_ptr,
    clippy::transmute_ptr_to_ref,
    clippy::large_types_passed_by_value,
    clippy::wildcard_dependencies,
    clippy::exit,
    clippy::match_wild_err_arm,
    clippy::linkedlist,
    clippy::option_map_or_none,
    clippy::manual_bits,
    clippy::as_underscore,
    clippy::manual_clamp,
    clippy::modulo_arithmetic,
    clippy::mem_forget,
    clippy::assign_op_pattern,
    clippy::iter_not_returning_iterator,
    clippy::needless_borrow,
    clippy::borrow_deref_ref,
    clippy::manual_string_new,
    clippy::use_self,
    clippy::inconsistent_struct_constructor,
    clippy::or_fun_call,
    clippy::single_match_else,
    clippy::redundant_slicing,
    clippy::string_lit_as_bytes,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_allocation,
    clippy::vec_resize_to_zero,
    clippy::zero_sized_map_values,
    clippy::large_enum_variant,
    clippy::mutable_key_type,
    clippy::ref_option_ref,
    clippy::trivially_copy_pass_by_ref,
    clippy::match_overlapping_arm,
    clippy::manual_range_contains,
    clippy::manual_find_map,
    clippy::redundant_pub_crate,
    clippy::redundant_else,
    clippy::undocumented_unsafe_blocks,
    clippy::cast_ptr_alignment,
    clippy::cast_lossless,
    clippy::as_conversions,
    clippy::missing_const_for_fn,
    clippy::borrowed_box,
    clippy::extend_with_drain,
    clippy::clone_on_copy,
    dropping_references,
    clippy::len_zero,
    clippy::implicit_hasher,
    clippy::map_clone,
    clippy::useless_asref
)]

/// Core, is logic of Command Line application.
pub mod core {
    /// To match a element of website
    pub mod element;

    /// To find element of website and show
    /// all infomation, if this exist
    pub mod find_element;

    /// Scraper all href URL in website
    pub mod crawl_href;
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
