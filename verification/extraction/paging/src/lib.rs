// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root for the page-descriptor backends. The real kernel source is
//! included via #[path]; nothing is copied or retyped.
//!
//! The aarch64 builders are reached through forwarding roots because a
//! `--start-from` naming an item re-exported out of a private module does not
//! resolve, the same device the `policy` and `iommu` crates use.

pub mod arch;

pub fn aarch64_leaf(pa: u64, flags: u64) -> u64 {
    arch::paging::descriptor::aarch64::leaf(pa, flags)
}

pub fn aarch64_table(pa: u64, user_accessible: bool) -> u64 {
    arch::paging::descriptor::aarch64::table(pa, user_accessible)
}
