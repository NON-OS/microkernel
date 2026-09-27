// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/cap/agaw.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/cap/agaw.rs"]
pub mod agaw;

pub fn agawlevels_page_table_levels(this: agaw::AgawLevels) -> u8 {
    this.page_table_levels()
}

pub fn agawlevels_context_aw(this: agaw::AgawLevels) -> u8 {
    this.context_aw()
}

