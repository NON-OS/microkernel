// NONOS Operating System (AGPL-3.0-or-later)
//! The pipeline text filters, `grep`'s match marks and the `tree` drawing.

/// The pipeline text filters. `pub(super)` in the capsule, which resolves to
/// visible from this group here, so the tests can reach them. Mounted for the
/// tests only, since nothing else here calls them.
#[cfg(test)]
#[path = "../../capsule_terminal/src/command/dispatch/filter/text.rs"]
pub mod text;

/// Each filter's flag table, the lines a file gives a filter, and what a pipe
/// stage that is not a filter says. `text` reads its flags from here.
#[path = "../../capsule_terminal/src/command/dispatch/filter/input.rs"]
pub mod input;

/// `wc`, `head` and `tail` over piped lines; crate-private in the capsule, so
/// compiled for the tests alone.
#[cfg(test)]
#[path = "../../capsule_terminal/src/command/dispatch/filter/count.rs"]
pub mod count;

// Reached only from the tests; the capsule calls it from grep_scan. Mounted for
// the tests only, as the filters are.
#[cfg(test)]
#[path = "../../capsule_terminal/src/command/builtin/fs/grep_paint.rs"]
pub mod grep_paint;

#[path = "../../capsule_terminal/src/command/builtin/fs/tree_render.rs"]
pub mod tree_render;

#[cfg(test)]
mod filter_tests;
#[cfg(test)]
mod grep_paint_tests;
#[cfg(test)]
mod nl_tests;
#[cfg(test)]
mod pipe_filter_tests;
#[cfg(test)]
mod tree_tests;
