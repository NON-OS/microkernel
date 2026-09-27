// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/region/types/region_type.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/region/types/region_type.rs"]
pub mod region_type;

pub fn regiontype_is_allocatable(this: region_type::RegionType) -> bool {
    this.is_allocatable()
}

pub fn regiontype_is_kernel(this: region_type::RegionType) -> bool {
    this.is_kernel()
}

pub fn regiontype_is_reserved(this: region_type::RegionType) -> bool {
    this.is_reserved()
}

