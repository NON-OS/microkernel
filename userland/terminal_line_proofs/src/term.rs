// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's `term` module, as far as the sources under test reach
//! into it: the one set of dimensions they read, and the real helpers.

pub mod dimensions {
    pub const COLS: usize = 96;
    pub const LINE_MAX: usize = 1024;
    pub const HISTORY_DEPTH: usize = 32;
    pub const SCROLLBACK_ROWS: usize = 3000;
    pub const VISIBLE_ROWS: usize = 40;
}

/// The screen and its history, the real one.
pub use crate::scrollback;

/// The themes, at the path the capsule names them by.
pub use crate::theme;

pub mod util {
    pub use crate::fmt_u64::format_u64;
    pub use crate::qwen::space::is_space;
}
