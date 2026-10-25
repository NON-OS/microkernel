// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

//! Why there is no boot framebuffer, from the handoff (H-06(a)). With
//! flags::FB_REFUSED set and FB_AVAILABLE clear, the loader put its reason in
//! fb.pixel_format (nonos-bootloader display/gop/refusal_word.rs): bits 0 to 7
//! a NO_FB_* code, bits 8 to 15 a refused bitmask mode's bits per pixel, the
//! rest 0. Both flags set is malformed (security/framebuffer.rs);
//! a code or bit this does not know is Unknown, named and never trusted; an
//! older loader sets neither flag and gives no reason. Pure:
//! kernel_proofs::handoff_refusal.

use super::constants::flags;

/// The loader's codes, as its refusal_word.rs has them (held equal in proofs).
pub const NO_FB_NO_GOP: u8 = 0;
pub const NO_FB_BLT_ONLY: u8 = 1;
pub const NO_FB_BITMASK: u8 = 2;
pub const NO_FB_BAD_GEOMETRY: u8 = 3;
/// Beyond the loader's codes, for userland (nonos_display_refusal): the kernel
/// refused the framebuffer itself, did not trust the reason, or got none.
pub const WHY_NOT_MAPPED: u8 = 0xFD;
pub const WHY_UNKNOWN: u8 = 0xFE;
pub const WHY_NONE_GIVEN: u8 = 0xFF;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FbRefusal {
    NoGop,
    BltOnly,
    Bitmask { bpp: u8 },
    BadGeometry,
    Unknown(u32),
    NoneGiven,
}

/// A handoff claiming a framebuffer and a refusal both.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Malformed;

/// Ok(None) when a framebuffer was handed over.
pub fn refusal(handoff_flags: u64, word: u32) -> Result<Option<FbRefusal>, Malformed> {
    let available = handoff_flags & flags::FB_AVAILABLE != 0;
    let refused = handoff_flags & flags::FB_REFUSED != 0;
    match (available, refused) {
        (true, true) => Err(Malformed),
        (true, false) => Ok(None),
        (false, false) => Ok(Some(FbRefusal::NoneGiven)),
        (false, true) => Ok(Some(decode(word))),
    }
}

fn decode(word: u32) -> FbRefusal {
    let (code, bpp) = (word as u8, (word >> 8) as u8);
    match code {
        _ if word >> 16 != 0 => FbRefusal::Unknown(word),
        NO_FB_NO_GOP if bpp == 0 => FbRefusal::NoGop,
        NO_FB_BLT_ONLY if bpp == 0 => FbRefusal::BltOnly,
        NO_FB_BITMASK => FbRefusal::Bitmask { bpp },
        NO_FB_BAD_GEOMETRY if bpp == 0 => FbRefusal::BadGeometry,
        _ => FbRefusal::Unknown(word),
    }
}
