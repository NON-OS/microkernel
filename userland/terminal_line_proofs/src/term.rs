// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's `term` module, as far as the sources under test reach
//! into it: the one set of dimensions they read, and the real helpers.

pub mod dimensions {
    pub const COLS: usize = 96;
    pub const LINE_MAX: usize = 1024;
}

pub mod util {
    pub use crate::fmt_u64::format_u64;
    pub use crate::qwen::space::is_space;
}
