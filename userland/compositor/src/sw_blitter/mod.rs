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

pub mod copy_rect;
pub mod downscale;
pub mod fill;
pub mod fit;
pub mod magnify;
pub mod resample;
pub mod row;
pub mod scaled;
pub mod stretch;
pub mod upscale;

pub use copy_rect::{composite_layer, composite_with};
pub use downscale::downscale;
pub use fill::fill_rect;
pub use fit::fitted;
pub use magnify::magnify;
pub use resample::resample;
pub use scaled::scaled;
pub use upscale::upscale;

#[derive(Clone, Copy, Default)]
pub struct Surface {
    pub base_va: u64,
    pub stride: u32,
    pub width: u32,
    pub height: u32,
    pub byte_len: u64,
}

impl Surface {
    /// The box `(x, y, w, h)` of this surface as a surface of its own, cut
    /// to it: its rows start where the box does and end where this one's
    /// bytes do, so `row_start` holds every read of it inside this surface.
    /// None when nothing of the box is on it.
    pub fn view(self, (x, y, w, h): (u32, u32, u32, u32)) -> Option<Surface> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let (width, height) = (w.min(self.width - x), h.min(self.height - y));
        if width == 0 || height == 0 {
            return None;
        }
        let at = u64::from(y).checked_mul(u64::from(self.stride))?.checked_add(u64::from(x) * 4)?;
        let byte_len = self.byte_len.checked_sub(at).filter(|&n| n > 0)?;
        let base_va = self.base_va.checked_add(at)?;
        Some(Surface { base_va, stride: self.stride, width, height, byte_len })
    }

    pub fn row_start(self, y: u32, x: u32, width: u32) -> Option<usize> {
        if width == 0 || y >= self.height || x >= self.width || width > self.width - x {
            return None;
        }
        let row_off = (y as u64).checked_mul(self.stride as u64)?;
        let col_off = (x as u64).checked_mul(4)?;
        let span_bytes = (width as u64).checked_mul(4)?;
        let start = row_off.checked_add(col_off)?;
        let end = start.checked_add(span_bytes)?;
        if end > self.byte_len {
            return None;
        }
        usize::try_from(self.base_va.checked_add(start)?).ok()
    }
}
