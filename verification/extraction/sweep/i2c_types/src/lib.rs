// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/devices/i2c/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/devices/i2c/types.rs"]
pub mod types;

pub fn i2chiddevice_is_touchpad(this: types::I2cHidDevice) -> bool {
    this.is_touchpad()
}

pub fn i2chiddevice_is_touchscreen(this: types::I2cHidDevice) -> bool {
    this.is_touchscreen()
}

