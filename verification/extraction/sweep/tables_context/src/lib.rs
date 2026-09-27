// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/tables/context.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/tables/context.rs"]
pub mod context;

pub fn root_low(context_table_phys: u64) -> u64 {
    context::root_low(context_table_phys)
}

pub fn context_low(sl_root_phys: u64) -> u64 {
    context::context_low(sl_root_phys)
}

pub fn context_high(domain_id: u16, address_width: u8) -> u64 {
    context::context_high(domain_id, address_width)
}

pub fn context_index(device: u8, function: u8) -> usize {
    context::context_index(device, function)
}

pub fn is_present(low: u64) -> bool {
    context::is_present(low)
}

pub fn entry_address(low: u64) -> u64 {
    context::entry_address(low)
}

pub fn context_domain(high: u64) -> u16 {
    context::context_domain(high)
}

