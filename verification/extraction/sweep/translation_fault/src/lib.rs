// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/mmu/translation/fault.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/mmu/translation/fault.rs"]
pub mod fault;

pub fn translationfault_is_translation_fault(this: fault::TranslationFault) -> bool {
    this.is_translation_fault()
}

pub fn translationfault_is_access_fault(this: fault::TranslationFault) -> bool {
    this.is_access_fault()
}

pub fn translationfault_is_permission_fault(this: fault::TranslationFault) -> bool {
    this.is_permission_fault()
}

pub fn translationfault_level(this: fault::TranslationFault) -> u8 {
    this.level()
}

