//! Define all crates
#![deny(
    clippy::pedantic,
    clippy::all,
    clippy::nursery,
    clippy::perf,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
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
    clippy::useless_asref,
    clippy::redundant_clone,
    clippy::string_extend_chars,
    clippy::unnecessary_box_returns,
    clippy::needless_collect,
    clippy::large_stack_frames,
    clippy::manual_flatten,
    clippy::manual_filter_map,
    clippy::unnecessary_sort_by,
    clippy::unnecessary_wraps,
    clippy::bind_instead_of_map,
    clippy::unnecessary_operation,
    clippy::needless_continue,
    clippy::mem_replace_with_default,
    clippy::mem_replace_option_with_none,
    clippy::unused_async,
    clippy::unnecessary_unwrap,
    clippy::fallible_impl_from,
    clippy::option_as_ref_deref,
    clippy::redundant_else,
    clippy::redundant_closure_call,
    clippy::unused_unit,
    clippy::needless_borrow,
    clippy::borrowed_box,
    clippy::default_numeric_fallback,
    clippy::suspicious_operation_groupings,
    clippy::unused_io_amount,
    clippy::redundant_feature_names,
    clippy::redundant_pattern,
    clippy::inefficient_to_string,
    clippy::semicolon_inside_block,
    clippy::redundant_type_annotations,
    clippy::explicit_into_iter_loop,
    clippy::manual_split_once,
    clippy::mutex_integer,
    clippy::match_like_matches_macro,
    clippy::bool_assert_comparison,
    clippy::mutable_key_type,
    clippy::iter_on_single_items,
    clippy::str_to_string,
    clippy::redundant_clone,
    clippy::excessive_precision,
    unused_results,
    clippy::missing_inline_in_public_items,
    clippy::missing_assert_message,
    clippy::wildcard_enum_match_arm,
    clippy::string_add,
    clippy::modulo_arithmetic,
    clippy::indexing_slicing,
    clippy::integer_division,
    clippy::mem_forget,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::inline_asm_x86_att_syntax,
    clippy::unreachable,
    clippy::exit,
    clippy::undocumented_unsafe_blocks,
    clippy::string_add_assign,
    clippy::unnecessary_cast,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::shadow_reuse,
    clippy::string_lit_as_bytes,
    clippy::borrow_as_ptr,
    clippy::wildcard_imports,
    clippy::panic,
    clippy::float_cmp_const,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::mut_mut,
    clippy::semicolon_if_nothing_returned,
    clippy::unnecessary_safety_comment,
    clippy::verbose_file_reads,
    clippy::str_to_string,
    clippy::large_stack_arrays,
    clippy::match_bool,
    clippy::redundant_pub_crate,
    clippy::ptr_as_ptr,
    clippy::use_debug,
    clippy::default_trait_access,
    clippy::if_then_some_else_none,
    clippy::lossy_float_literal,
    clippy::redundant_else,
    clippy::large_enum_variant,
    clippy::needless_pass_by_value,
    clippy::large_types_passed_by_value,
    clippy::default_numeric_fallback,
    clippy::redundant_slicing,
    clippy::trivially_copy_pass_by_ref,
    clippy::option_if_let_else,
    clippy::manual_map,
    clippy::inconsistent_struct_constructor,
    clippy::match_overlapping_arm,
    clippy::match_like_matches_macro,
    clippy::manual_bits,
    clippy::manual_clamp,
    clippy::redundant_closure_call,
    clippy::redundant_closure,
    clippy::redundant_type_annotations,
    clippy::unnecessary_operation,
    clippy::manual_let_else,
    clippy::needless_continue,
    clippy::redundant_feature_names,
    clippy::redundant_pattern,
    clippy::redundant_static_lifetimes,
    clippy::bool_assert_comparison,
    clippy::iter_not_returning_iterator,
    clippy::suspicious_operation_groupings,
    clippy::ptr_arg,
    clippy::useless_asref,
    clippy::manual_memcpy,
    clippy::borrow_deref_ref,
    clippy::unnecessary_wraps,
    clippy::explicit_into_iter_loop,
    clippy::unnecessary_unwrap,
    clippy::fallible_impl_from,
    clippy::borrowed_box,
    clippy::option_as_ref_deref,
    clippy::inline_always,
    clippy::empty_enum,
    clippy::empty_structs_with_brackets,
    clippy::multiple_inherent_impl,
    clippy::manual_find_map,
    clippy::mutex_integer,
    clippy::match_wild_err_arm,
    clippy::option_map_or_none,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_allocation,
    clippy::redundant_field_names,
    clippy::redundant_closure_for_method_calls,
    clippy::manual_strip,
    clippy::trivial_regex,
    clippy::unnecessary_lazy_evaluations,
    clippy::slow_vector_initialization,
    clippy::vec_box,
    clippy::large_futures,
    clippy::bind_instead_of_map,
    clippy::manual_flatten,
    clippy::manual_filter_map,
    clippy::unnecessary_sort_by,
    clippy::needless_collect,
    clippy::needless_borrow,
    clippy::string_extend_chars,
    clippy::large_stack_frames,
    clippy::mem_replace_with_default,
    clippy::mem_replace_option_with_none,
    clippy::manual_range_contains,
    clippy::cast_ptr_alignment,
    clippy::cast_lossless,
    clippy::as_conversions,
    clippy::unnecessary_box_returns,
    clippy::redundant_else,
    clippy::unused_async,
    clippy::iter_on_single_items,
    clippy::redundant_clone,
    clippy::redundant_pattern_matching,
    clippy::unnecessary_to_owned,
    clippy::use_self,
    clippy::zero_sized_map_values,
    clippy::mutable_key_type,
    clippy::redundant_type_annotations,
    clippy::redundant_pub_crate,
    clippy::redundant_closure,
    clippy::manual_string_new,
    clippy::or_fun_call,
    clippy::single_match_else,
    clippy::assign_op_pattern,
    clippy::unnecessary_box_returns,
    clippy::string_slice,
    clippy::integer_division,
    clippy::mem_forget,
    clippy::indexing_slicing,
    clippy::float_cmp,
    clippy::transmute_ptr_to_ref,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::todo,
    clippy::unreachable,
    clippy::dbg_macro,
    clippy::modulo_arithmetic,
    clippy::manual_range_contains,
    clippy::redundant_pub_crate,
    clippy::redundant_type_annotations,
    clippy::redundant_closure,
    clippy::redundant_closure_call,
    clippy::redundant_else,
    clippy::redundant_slicing,
    clippy::redundant_allocation,
    clippy::redundant_field_names,
    clippy::redundant_static_lifetimes,
    clippy::redundant_pattern_matching,
    clippy::redundant_feature_names,
    clippy::needless_borrow,
    clippy::needless_pass_by_value,
    clippy::needless_continue,
    clippy::needless_collect,
    clippy::manual_map,
    clippy::manual_memcpy,
    clippy::manual_string_new,
    clippy::manual_clamp,
    clippy::manual_bits,
    clippy::manual_let_else,
    clippy::manual_assert,
    clippy::manual_find_map,
    clippy::manual_flatten,
    clippy::manual_filter_map,
    clippy::manual_split_once,
    clippy::large_enum_variant,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::large_futures,
    clippy::shadow_unrelated,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::wildcard_imports,
    clippy::wildcard_enum_match_arm,
    clippy::wildcard_dependencies,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_assert_message,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc,
    clippy::missing_const_for_fn,
    clippy::missing_inline_in_public_items,
    clippy::match_bool,
    clippy::match_overlapping_arm,
    clippy::match_wild_err_arm,
    clippy::match_like_matches_macro,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::mutex_atomic,
    clippy::rc_mutex,
    clippy::rc_buffer,
    clippy::ptr_as_ptr,
    clippy::borrow_as_ptr,
    clippy::borrowed_box,
    clippy::borrow_deref_ref,
    clippy::exit,
    clippy::panic,
    clippy::verbose_file_reads,
    clippy::use_debug,
    clippy::use_self,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::string_lit_as_bytes,
    clippy::string_slice,
    clippy::string_extend_chars,
    clippy::str_to_string,
    clippy::implicit_clone,
    clippy::implicit_hasher,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::inconsistent_struct_constructor,
    clippy::trivially_copy_pass_by_ref,
    clippy::trivial_regex,
    clippy::len_zero,
    clippy::bool_assert_comparison,
    clippy::default_numeric_fallback,
    clippy::default_trait_access,
    clippy::as_conversions,
    clippy::as_underscore,
    clippy::cast_ptr_alignment,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::option_if_let_else,
    clippy::option_map_or_none,
    clippy::option_as_ref_deref,
    clippy::unwrap_or_default,
    clippy::fallible_impl_from,
    clippy::excessive_precision,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::inline_always,
    clippy::explicit_into_iter_loop,
    clippy::iter_on_single_items,
    clippy::iter_not_returning_iterator,
    clippy::map_clone,
    clippy::bind_instead_of_map,
    clippy::or_fun_call,
    clippy::assign_op_pattern,
    clippy::useless_asref,
    clippy::unnecessary_lazy_evaluations,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_operation,
    clippy::unnecessary_box_returns,
    clippy::unnecessary_sort_by,
    clippy::unnecessary_to_owned,
    clippy::unnecessary_wraps,
    clippy::unnecessary_cast,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_lazy_evaluations,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_operation,
    clippy::unnecessary_sort_by,
    clippy::unnecessary_wraps,
    clippy::unnecessary_to_owned,
    clippy::unnecessary_box_returns,
    clippy::unnecessary_cast,
    clippy::unnecessary_safety_comment
)]

/// Core, is logic of Command Line application.
pub mod core {
    /// Logging system
    pub mod log;

    /// For http request/api testing
    pub mod http {
        /// GET request
        pub mod get;

        /// POST request
        pub mod post;

        /// PUT request
        pub mod put;

        /// DELETE request
        pub mod delete;
    }
}

/// CLI module, to define all commands of CLI
mod cli;

/// For handles all commands of CLI
pub mod cmds {
    /// Handles all commands in this module
    pub mod handles;
}

/// Enum helper
pub mod ultis {
    /// For content type, such as "application/json", "text/html", etc...
    pub mod content_type;

    /// For generate setting
    pub mod setting;
}

/// Define Colors/LogLevel
pub mod colors;
