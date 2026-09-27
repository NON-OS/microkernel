// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/paging/error/fault_info.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/paging/error/fault_info.rs"]
pub mod fault_info;

pub fn pagefaultinfo_from_fault(address: u64, error_code: u64) -> fault_info::PageFaultInfo {
    fault_info::PageFaultInfo::from_fault(address, error_code)
}

pub fn pagefaultinfo_is_cow_fault(this: fault_info::PageFaultInfo) -> bool {
    this.is_cow_fault()
}

pub fn pagefaultinfo_is_demand_fault(this: fault_info::PageFaultInfo) -> bool {
    this.is_demand_fault()
}

