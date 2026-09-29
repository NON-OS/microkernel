// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/multiboot/structures_header.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn multiboot2header_new(header_length: u32) -> crate::arch::x86_64::multiboot::structures_header::Multiboot2Header {
    crate::arch::x86_64::multiboot::structures_header::Multiboot2Header::new(header_length)
}

pub fn multiboot2header_verify_checksum(this: crate::arch::x86_64::multiboot::structures_header::Multiboot2Header) -> bool {
    this.verify_checksum()
}

