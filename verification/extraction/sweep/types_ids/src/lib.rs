// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/types/ids.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/types/ids.rs"]
pub mod ids;

pub fn domainid_new(id: u16) -> ids::DomainId {
    ids::DomainId::new(id)
}

pub fn domainid_as_u16(this: ids::DomainId) -> u16 {
    this.as_u16()
}

pub fn sourceid_new(raw: u16) -> ids::SourceId {
    ids::SourceId::new(raw)
}

pub fn sourceid_as_u16(this: ids::SourceId) -> u16 {
    this.as_u16()
}

pub fn sourceid_bus(this: ids::SourceId) -> u8 {
    this.bus()
}

pub fn sourceid_device(this: ids::SourceId) -> u8 {
    this.device()
}

pub fn sourceid_function(this: ids::SourceId) -> u8 {
    this.function()
}

