// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root for the IOMMU table encodings. The real kernel source is
//! included via #[path]; nothing is copied or retyped.
//!
//! The two functions below are call-graph roots that forward to the real
//! methods, the same device the `policy` crate uses, because a `--start-from`
//! naming an inherent method on an enum does not resolve.

pub mod arch;

use arch::x86_64::iommu::regs::cap::agaw::AgawLevels;

pub fn agaw_page_table_levels(levels: AgawLevels) -> u8 {
    levels.page_table_levels()
}

pub fn agaw_context_aw(levels: AgawLevels) -> u8 {
    levels.context_aw()
}
