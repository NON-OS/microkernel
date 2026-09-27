// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/elf/loader/core/section.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/elf/loader/core/section.rs"]
pub mod section;

pub fn parsedsection_is_alloc(this: section::ParsedSection) -> bool {
    this.is_alloc()
}

pub fn parsedsection_is_symtab(this: section::ParsedSection) -> bool {
    this.is_symtab()
}

pub fn parsedsection_is_strtab(this: section::ParsedSection) -> bool {
    this.is_strtab()
}

pub fn parsedsection_is_rela(this: section::ParsedSection) -> bool {
    this.is_rela()
}

