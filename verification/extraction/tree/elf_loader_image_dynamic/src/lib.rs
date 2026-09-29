// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/elf/loader/image/dynamic.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod elf;

pub mod memory;

pub fn dynamicinfo_new() -> crate::elf::loader::image::dynamic::DynamicInfo {
    crate::elf::loader::image::dynamic::DynamicInfo::new()
}

pub fn dynamicinfo_needs_relocation(this: crate::elf::loader::image::dynamic::DynamicInfo) -> bool {
    this.needs_relocation()
}

pub fn dynamicinfo_needs_linking(this: crate::elf::loader::image::dynamic::DynamicInfo) -> bool {
    this.needs_linking()
}

pub fn dynamicinfo_rela_count(this: crate::elf::loader::image::dynamic::DynamicInfo) -> usize {
    this.rela_count()
}

pub fn dynamicinfo_plt_rela_count(this: crate::elf::loader::image::dynamic::DynamicInfo) -> usize {
    this.plt_rela_count()
}

