// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Why the loader latched no framebuffer, as it tells the kernel: with
//! BootHandoffV1 flags::FB_REFUSED set and FB_AVAILABLE clear, fb.pixel_format
//! is this word. Bits 0 to 7 are a NO_FB_* code; bits 8 to 15 the bits per
//! pixel of the deepest PixelBitMask mode refused, given with NO_FB_BITMASK
//! alone and 0 otherwise; the rest are 0. The kernel's decoder is
//! src/boot/handoff/types/refusal.rs; kernel_proofs::handoff_refusal mounts
//! both and round-trips every word this makes. Pure.

/// No handle carries the Graphics Output Protocol at all.
pub const NO_FB_NO_GOP: u8 = 0;
/// Every GOP mode is PixelBltOnly: the device has no linear framebuffer
/// the CPU can address, only the firmware's Blt() call, which is gone
/// after ExitBootServices.
pub const NO_FB_BLT_ONLY: u8 = 1;
/// Linear modes exist but only in a PixelBitMask layout other than 32 bit
/// RGB or BGR (16 bpp, 10 bit channels), which nothing here writes.
pub const NO_FB_BITMASK: u8 = 2;
/// The firmware reported a framebuffer smaller than the mode, or at 0.
pub const NO_FB_BAD_GEOMETRY: u8 = 3;

/// The word for `code`, with `bitmask_bpp` kept only for NO_FB_BITMASK.
pub const fn refusal_word(code: u8, bitmask_bpp: u8) -> u32 {
    let bpp = if code == NO_FB_BITMASK { bitmask_bpp as u32 } else { 0 };
    code as u32 | bpp << 8
}

/// The bits a PixelBitMask mode spends on a pixel: every bit its four masks name.
pub const fn mask_bits(red: u32, green: u32, blue: u32, reserved: u32) -> u8 {
    (red | green | blue | reserved).count_ones() as u8
}
