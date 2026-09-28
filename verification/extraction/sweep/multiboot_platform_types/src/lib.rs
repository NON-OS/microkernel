// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/multiboot/platform_types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/multiboot/platform_types.rs"]
pub mod platform_types;

pub fn platform_is_virtual(this: platform_types::Platform) -> bool {
    this.is_virtual()
}

pub fn platform_is_qemu(this: platform_types::Platform) -> bool {
    this.is_qemu()
}

pub fn platform_has_hw_virtualization(this: platform_types::Platform) -> bool {
    this.has_hw_virtualization()
}

pub fn platform_supports_virtio(this: platform_types::Platform) -> bool {
    this.supports_virtio()
}

pub fn platform_timer_frequency(this: platform_types::Platform) -> u32 {
    this.timer_frequency()
}

