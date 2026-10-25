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

//! What the compositor says while no display has come up (H-06(a)). The
//! first failed ask says why at once; if none has come up 10 s on, it says so
//! once more, so a machine with no screen still leaves a named failure in the
//! log `log DISPLAY` reads. It keeps waiting either way: a display driver
//! that answers later still takes over. Pure: compositor_proofs.

/// How long after the first failed ask the second line is said.
pub const LOG_AFTER_MS: i64 = 10_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Note {
    Say,
    Log,
    Quiet,
}

/// What to say `waited_ms` after the first failed ask, given what was said.
pub fn note(waited_ms: i64, said: bool, logged: bool) -> Note {
    if !said {
        Note::Say
    } else if !logged && waited_ms >= LOG_AFTER_MS {
        Note::Log
    } else {
        Note::Quiet
    }
}

/// The reason in words, from nonos_display_refusal's word (libc
/// graphics/display_dimensions.rs); bits 8 to 15 are said by the caller.
pub fn reason(word: u32) -> &'static str {
    match word & 0xFF {
        0 => "the firmware has no Graphics Output Protocol",
        1 => "the firmware's display is PixelBltOnly, with no linear framebuffer",
        2 => "the firmware offers only PixelBitMask modes NONOS cannot draw",
        3 => "the firmware's framebuffer does not cover its own mode",
        0xFD => "the kernel would not map the firmware's framebuffer",
        0xFE => "the loader gave a reason the kernel does not trust",
        0xFF => "the loader gave no reason",
        _ => "the kernel gave a reason this compositor does not know",
    }
}
