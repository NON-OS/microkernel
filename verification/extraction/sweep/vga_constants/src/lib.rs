// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/vga/constants.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/vga/constants.rs"]
pub mod constants;

pub fn colorcode_foreground(this: constants::ColorCode) -> u8 {
    this.foreground()
}

pub fn colorcode_background(this: constants::ColorCode) -> u8 {
    this.background()
}

pub fn colorcode_is_blinking(this: constants::ColorCode) -> bool {
    this.is_blinking()
}

pub fn colorcode_value(this: constants::ColorCode) -> u8 {
    this.value()
}

pub fn screenchar_as_u16(this: constants::ScreenChar) -> u16 {
    this.as_u16()
}


pub fn colorcode_new(foreground: constants::Color, background: constants::Color) -> constants::ColorCode {
    constants::ColorCode::new(foreground, background)
}
