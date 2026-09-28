// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/idt/entry_frame.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/idt/entry_frame.rs"]
pub mod entry_frame;

pub fn interruptframe_from_user(this: entry_frame::InterruptFrame) -> bool {
    this.from_user()
}

pub fn interruptframe_from_kernel(this: entry_frame::InterruptFrame) -> bool {
    this.from_kernel()
}

pub fn pagefaulterror_protection_violation(this: entry_frame::PageFaultError) -> bool {
    this.protection_violation()
}

pub fn pagefaulterror_write(this: entry_frame::PageFaultError) -> bool {
    this.write()
}

pub fn pagefaulterror_user(this: entry_frame::PageFaultError) -> bool {
    this.user()
}

pub fn pagefaulterror_reserved_write(this: entry_frame::PageFaultError) -> bool {
    this.reserved_write()
}

pub fn pagefaulterror_instruction_fetch(this: entry_frame::PageFaultError) -> bool {
    this.instruction_fetch()
}

pub fn pagefaulterror_protection_key(this: entry_frame::PageFaultError) -> bool {
    this.protection_key()
}

pub fn pagefaulterror_shadow_stack(this: entry_frame::PageFaultError) -> bool {
    this.shadow_stack()
}

pub fn pagefaulterror_sgx(this: entry_frame::PageFaultError) -> bool {
    this.sgx()
}

