// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/drivers/pci/error.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/drivers/pci/error.rs"]
pub mod error;

pub fn pcierror_is_fatal(this: error::PciError) -> bool {
    this.is_fatal()
}

pub fn pcierror_is_security_related(this: error::PciError) -> bool {
    this.is_security_related()
}

pub fn pcierror_is_recoverable(this: error::PciError) -> bool {
    this.is_recoverable()
}

