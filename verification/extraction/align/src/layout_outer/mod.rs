// NONOS Operating System (AGPL-3.0-or-later)
//! Nesting that lets `align.rs` find the constant it imports as
//! `super::super::constants::PAGE_SIZE_U64`.
pub mod constants {
    pub const PAGE_SIZE_U64: u64 = 4096;
}
pub mod manager;
