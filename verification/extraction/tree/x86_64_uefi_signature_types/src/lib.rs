// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/uefi/signature/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn signatureentry_data_len(this: crate::arch::x86_64::uefi::signature::types::SignatureEntry) -> usize {
    this.data_len()
}

pub fn signatureentry_total_size(this: crate::arch::x86_64::uefi::signature::types::SignatureEntry) -> usize {
    this.total_size()
}

pub fn signaturelist_entry_count(this: crate::arch::x86_64::uefi::signature::types::SignatureList) -> usize {
    this.entry_count()
}

pub fn signaturelist_is_empty(this: crate::arch::x86_64::uefi::signature::types::SignatureList) -> bool {
    this.is_empty()
}

pub fn signaturelist_signature_size(this: crate::arch::x86_64::uefi::signature::types::SignatureList) -> usize {
    this.signature_size()
}

pub fn signaturelist_total_size(this: crate::arch::x86_64::uefi::signature::types::SignatureList) -> usize {
    this.total_size()
}

