// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/uefi/types/guid.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn guid_null() -> crate::arch::x86_64::uefi::types::guid::Guid {
    crate::arch::x86_64::uefi::types::guid::Guid::null()
}

pub fn guid_is_null(this: crate::arch::x86_64::uefi::types::guid::Guid) -> bool {
    this.is_null()
}

pub fn guid_is_hash_type(this: crate::arch::x86_64::uefi::types::guid::Guid) -> bool {
    this.is_hash_type()
}

pub fn guid_is_certificate_type(this: crate::arch::x86_64::uefi::types::guid::Guid) -> bool {
    this.is_certificate_type()
}

