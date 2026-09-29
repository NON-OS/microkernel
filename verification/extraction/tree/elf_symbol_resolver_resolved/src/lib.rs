// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/elf/symbol/resolver/resolved.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod elf;

pub mod memory;

pub fn resolvedsymbol_is_function(this: crate::elf::symbol::resolver::resolved::ResolvedSymbol) -> bool {
    this.is_function()
}

pub fn resolvedsymbol_is_object(this: crate::elf::symbol::resolver::resolved::ResolvedSymbol) -> bool {
    this.is_object()
}

pub fn resolvedsymbol_is_global(this: crate::elf::symbol::resolver::resolved::ResolvedSymbol) -> bool {
    this.is_global()
}

pub fn resolvedsymbol_is_weak(this: crate::elf::symbol::resolver::resolved::ResolvedSymbol) -> bool {
    this.is_weak()
}

