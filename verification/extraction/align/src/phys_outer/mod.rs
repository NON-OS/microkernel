// NONOS Operating System (AGPL-3.0-or-later)
//! Nesting that lets `helpers.rs` find `super::pages`.
pub mod pages {
    pub const PAGE_SIZE_U64: u64 = 4096;
    pub const BITS_PER_BYTE: usize = 8;
}
#[path = "../../../../../src/memory/phys/constants/helpers.rs"]
pub mod helpers;
