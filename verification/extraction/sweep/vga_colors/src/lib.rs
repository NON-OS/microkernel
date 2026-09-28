// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/boot/vga/colors.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/boot/vga/colors.rs"]
pub mod colors;

pub fn make_attr(fg: u8, bg: u8) -> u8 {
    colors::make_attr(fg, bg)
}

pub fn fg_color(attr: u8) -> u8 {
    colors::fg_color(attr)
}

pub fn bg_color(attr: u8) -> u8 {
    colors::bg_color(attr)
}

