// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the terminal line editor. Includes the real capsule source
//! via `#[path]` so the code under test is the code that ships.

// The capsule sources are no_std and reach the heap through `alloc`. Declaring
// it here lets them compile unchanged against the host's std.
extern crate alloc;

/// The capsule's own module layout, mirrored so its sources compile unchanged.
/// Everything under here is either the real file or the one constant the
/// capsule reads from its own dimensions.
pub mod term {
    pub mod dimensions {
        pub const COLS: usize = 96;
    }
    pub mod util {
        pub use crate::fmt_u64::format_u64;
    }
}

// Declared at the crate root: a `#[path]` inside nested inline modules
// resolves against the directory those modules imply, not against this file.
#[path = "../../capsule_terminal/src/term/util/format_u64.rs"]
pub mod fmt_u64;

#[path = "../../capsule_terminal/src/command/flags/mod.rs"]
pub mod flags_inner;

pub mod command {
    pub use crate::flags_inner as flags;
}

/// The pipeline text filters. `pub(super)` in the capsule, which resolves to
/// crate-visible here, so the tests can reach them.
///
/// Everything in the module is reached from the tests and from nowhere else,
/// so on a plain library build it reads as dead. It is not: the capsule calls
/// all of it. The allowance covers the shape of this crate, not a real gap.
#[allow(dead_code)]
#[path = "../../capsule_terminal/src/command/dispatch/filter/text.rs"]
pub mod text;

#[path = "../../capsule_terminal/src/term/line/mod.rs"]
pub mod line;

#[path = "../../capsule_terminal/src/command/builtin/fs/tree_render.rs"]
pub mod tree_render;

#[path = "../../capsule_terminal/src/term/history/expand.rs"]
pub mod expand;

#[path = "../../capsule_terminal/src/command/suggest.rs"]
pub mod suggest;

// The editor shares this crate rather than growing a second proofs crate for
// two pure functions.
#[path = "../../capsule_text_editor/src/editor/goto_line.rs"]
pub mod goto_line;

#[path = "../../capsule_text_editor/src/editor/quick_open.rs"]
pub mod quick_open;

#[path = "../../capsule_net_core/src/server/runner/cadence.rs"]
pub mod cadence;

// Reached only from the tests on a plain library build; the capsule calls it
// from grep_scan. Same crate-shape allowance as the filters above.
#[allow(dead_code)]
#[path = "../../capsule_terminal/src/command/builtin/fs/grep_paint.rs"]
pub mod grep_paint;

#[cfg(test)]
mod cadence_tests;
#[cfg(test)]
mod expand_tests;
#[cfg(test)]
mod filter_tests;
#[cfg(test)]
mod goto_tests;
#[cfg(test)]
mod grep_paint_tests;
#[cfg(test)]
mod line_tests;
#[cfg(test)]
mod quick_open_tests;
#[cfg(test)]
mod suggest_tests;
#[cfg(test)]
mod tree_tests;
#[cfg(test)]
mod usage_tests;
