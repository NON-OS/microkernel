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

//! An FbRefusal said: as the word userland reads (nonos_display_refusal) and
//! in words for the boot log. Pure: kernel_proofs::handoff_refusal.

use super::refusal::{FbRefusal, WHY_NONE_GIVEN, WHY_UNKNOWN};

impl FbRefusal {
    /// Bits 0 to 7 the reason (a NO_FB_* code, WHY_UNKNOWN or
    /// WHY_NONE_GIVEN), bits 8 to 15 a refused bitmask mode's bits per pixel,
    /// the rest 0: never a word the kernel did not check.
    pub fn why_word(self) -> u32 {
        match self {
            Self::NoGop => 0,
            Self::BltOnly => 1,
            Self::Bitmask { bpp } => 2 | (bpp as u32) << 8,
            Self::BadGeometry => 3,
            Self::Unknown(_) => WHY_UNKNOWN as u32,
            Self::NoneGiven => WHY_NONE_GIVEN as u32,
        }
    }

    /// Why, after "no framebuffer handed over".
    pub fn says(self) -> &'static str {
        match self {
            Self::NoGop => ": the firmware has no Graphics Output Protocol",
            Self::BltOnly => ": the firmware's display is PixelBltOnly, with no linear framebuffer",
            Self::Bitmask { .. } => {
                ": the firmware offers only PixelBitMask modes NONOS cannot draw (bits a pixel:"
            }
            Self::BadGeometry => ": the firmware's framebuffer does not cover its own mode",
            Self::Unknown(_) => ": the loader gave a reason the kernel does not know (word:",
            Self::NoneGiven => " (the loader gave no reason)",
        }
    }
}
