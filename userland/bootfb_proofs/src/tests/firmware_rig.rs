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

//! A model framebuffer and the record the kernel would list for it.

use super::firmware_model::Model;
use crate::geometry::INFO_LEN;
use crate::record::{Listed, BAR_FLAG_PREFETCH, BAR_KIND_MMIO, BUS_KIND_FIRMWARE, CLASS_DISPLAY};

impl Model {
    /// A mode `w` x `h` at `stride` in `format`, its first pixel `offset`
    /// bytes into the bar's page; the bar ends at the last row's last pixel.
    pub fn new(w: u32, h: u32, stride: u32, format: u32, offset: u32) -> (Model, Listed) {
        let size = offset as u64 + stride as u64 * (h as u64 - 1) + w as u64 * 4;
        let mut info = [0u8; INFO_LEN];
        for (i, v) in [w, h, stride, format, offset, 300 | 170 << 16].iter().enumerate() {
            info[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        let bar = vec![0u32; size.div_ceil(4) as usize];
        let (calls, said) = (Vec::new(), Vec::new());
        let m = Model { bar, info, calls, said, announced: false, holder: false, taken: false };
        let bar0 = (BAR_KIND_MMIO, BAR_FLAG_PREFETCH, 0x8000_0000, size, offset);
        let (device_id, bus_kind, class) = (42, BUS_KIND_FIRMWARE, CLASS_DISPLAY);
        (m, Listed { device_id, bus_kind, class, vendor: 0, device: 1, bar_count: 1, bar0 })
    }

    pub fn byte(&self, at: usize) -> u8 {
        self.bar[at / 4].to_le_bytes()[at % 4]
    }
}
