// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/iommu/backend_unsupported_fault.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/iommu/backend_unsupported_fault.rs"]
pub mod backend_unsupported_fault;

pub fn block_faulted() {
    backend_unsupported_fault::block_faulted()
}

