// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/riscv64/mmu/attributes/flags.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/riscv64/mmu/attributes/flags.rs"]
pub mod flags;

pub fn pteflags_new() -> flags::PteFlags {
    flags::PteFlags::new()
}

pub fn pteflags_from_bits(bits: u64) -> flags::PteFlags {
    flags::PteFlags::from_bits(bits)
}

pub fn pteflags_valid(this: flags::PteFlags) -> flags::PteFlags {
    this.valid()
}

pub fn pteflags_readable(this: flags::PteFlags) -> flags::PteFlags {
    this.readable()
}

pub fn pteflags_writable(this: flags::PteFlags) -> flags::PteFlags {
    this.writable()
}

pub fn pteflags_executable(this: flags::PteFlags) -> flags::PteFlags {
    this.executable()
}

pub fn pteflags_user(this: flags::PteFlags) -> flags::PteFlags {
    this.user()
}

pub fn pteflags_global(this: flags::PteFlags) -> flags::PteFlags {
    this.global()
}

pub fn pteflags_accessed(this: flags::PteFlags) -> flags::PteFlags {
    this.accessed()
}

pub fn pteflags_dirty(this: flags::PteFlags) -> flags::PteFlags {
    this.dirty()
}

pub fn pteflags_bits(this: flags::PteFlags) -> u64 {
    this.bits()
}

pub fn pteflags_is_valid(this: flags::PteFlags) -> bool {
    this.is_valid()
}

pub fn pteflags_is_leaf(this: flags::PteFlags) -> bool {
    this.is_leaf()
}

pub fn pteflags_is_readable(this: flags::PteFlags) -> bool {
    this.is_readable()
}

pub fn pteflags_is_writable(this: flags::PteFlags) -> bool {
    this.is_writable()
}

pub fn pteflags_is_executable(this: flags::PteFlags) -> bool {
    this.is_executable()
}

pub fn pteflags_is_user(this: flags::PteFlags) -> bool {
    this.is_user()
}

