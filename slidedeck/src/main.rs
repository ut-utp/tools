//! A tool that turns text slides into an LC-3 program.

#![warn(clippy::pedantic)]
#![forbid(
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_lifetimes,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![deny(
    bad_style,
    unused,
    missing_debug_implementations,
    intra_doc_link_resolution_failure,
    missing_docs,
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    rust_2018_idioms
)]
#![doc(test(attr(deny(rust_2018_idioms, warnings))))]
#![doc(html_logo_url = "")] // TODO!

use lc3_shims::memory::FileBackedMemoryShim;

pub mod common;

mod ferris;
mod misc;

fn main() {
    FileBackedMemoryShim::with_initialized_memory("slides.mem", ferris::slide_deck())
        .flush()
        .map_err(|_| std::io::Error::last_os_error())
        .unwrap();
}
