// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/handover.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/handover.rs"]
pub mod handover;

pub fn amd_stop(backend: bool, dev: bool) -> bool {
    handover::amd_stop(backend, dev)
}

