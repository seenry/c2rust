#![feature(
    i128_type,
    rustc_private,
    trace_macros,
)]
extern crate arena;
extern crate diff;
extern crate rustc;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_metadata;
extern crate rustc_resolve;
extern crate rustc_trans;
extern crate syntax;
extern crate syntax_ext;
extern crate syntax_pos;

#[macro_use] mod macros;

mod util;
mod ast_equiv;
mod fold;
pub mod fold_node;
mod visit;
pub mod print_spans;
pub mod remove_paren;
mod cursor;
mod get_node_id;
mod get_span;
mod make_ast;
pub mod path_edit;
mod fn_edit;
mod dataflow;

pub mod pick_node;
pub mod mark_adjust;

mod bindings;
pub mod driver;
pub mod span_fix;
mod matcher;
mod matcher_impls;
mod seq_edit;
mod subst;
pub mod rewrite;
mod rewrite_impls;
pub mod file_rewrite;

mod api;
pub mod transform;
