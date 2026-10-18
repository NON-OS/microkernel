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

//! The mode a scanout is recorded at, and so the primary surface made at:
//! the device's own when it is at least 16x16 and its frame fits the
//! broker's display grant, else 1280x720. The size is the device's word: a
//! huge one (32767x32767 is about 4 GiB) used to be taken, and the surface's
//! DMA map then failed every bring-up attempt instead of falling back. A
//! small panel's own mode (320x240, say) used to be raised to 1280x720, a
//! frame it cannot show.

use crate::display::gpu::GRANT_BYTES;

/// What a scanout too small, too large or unknown is recorded at.
pub const DEFAULT: (u32, u32) = (1280, 720);
/// Below this a mode is not a display but a device's mistake.
const MIN: (u32, u32) = (16, 16);
const BYTES_PER_PIXEL: u64 = 4;

pub fn fit(width: u32, height: u32) -> (u32, u32) {
    let bytes = u64::from(width)
        .checked_mul(u64::from(height))
        .and_then(|px| px.checked_mul(BYTES_PER_PIXEL));
    let fits = matches!(bytes, Some(b) if b <= GRANT_BYTES);
    if width < MIN.0 || height < MIN.1 || !fits {
        return DEFAULT;
    }
    (width, height)
}
