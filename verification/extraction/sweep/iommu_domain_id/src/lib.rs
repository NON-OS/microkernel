// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/iommu/domain_id.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/iommu/domain_id.rs"]
pub mod domain_id;

pub fn domainid_new(id: u16) -> domain_id::DomainId {
    domain_id::DomainId::new(id)
}

pub fn domainid_as_u16(this: domain_id::DomainId) -> u16 {
    this.as_u16()
}

