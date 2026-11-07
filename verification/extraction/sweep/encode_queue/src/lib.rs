// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/iommu/smmu/encode/queue.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/iommu/smmu/encode/queue.rs"]
pub mod queue;

pub fn index(value: u32, log2: u8) -> u32 {
    queue::index(value, log2)
}

pub fn is_empty(prod: u32, cons: u32, log2: u8) -> bool {
    queue::is_empty(prod, cons, log2)
}

pub fn is_full(prod: u32, cons: u32, log2: u8) -> bool {
    queue::is_full(prod, cons, log2)
}

pub fn advance(value: u32, log2: u8) -> u32 {
    queue::advance(value, log2)
}

pub fn consumed(cons: u32, target: u32, log2: u8) -> bool {
    queue::consumed(cons, target, log2)
}

