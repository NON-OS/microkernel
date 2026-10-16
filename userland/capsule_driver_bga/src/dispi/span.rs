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

//! A mode's pixels within the framebuffer mapping the broker made.
//!
//! The mapping is as long as the adapter's BAR0, which is device-reported:
//! a std-VGA with 1 or 2 MiB of video memory (QEMU `vgamem_mb`) has a BAR0
//! shorter than a 1024x768x32 mode. A `Span` exists only for a mode that
//! fits, so every pixel written through one lies inside the mapping.

use crate::constants::BYTES_PER_PIXEL;
use crate::error::{BgaError, BgaResult};

pub struct Span {
    pub(super) va: u64,
    pub(super) pixels: u64,
}

/// The `width` x `height` mode in the `len` bytes mapped at `va`, or
/// `ModeTooLarge` when its pixels do not fit.
pub fn span(va: u64, len: u64, width: u16, height: u16) -> BgaResult<Span> {
    let pixels = (width as u64).checked_mul(height as u64).ok_or(BgaError::ModeTooLarge)?;
    let bytes = pixels.checked_mul(BYTES_PER_PIXEL as u64).ok_or(BgaError::ModeTooLarge)?;
    if bytes > len {
        return Err(BgaError::ModeTooLarge);
    }
    Ok(Span { va, pixels })
}
