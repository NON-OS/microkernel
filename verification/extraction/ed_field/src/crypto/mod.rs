// NONOS Operating System (AGPL-3.0-or-later)
//! Real directories, because a `#[path]` inside a nested inline module resolves
//! against a directory that does not exist and cannot walk out of it.
pub mod util;
