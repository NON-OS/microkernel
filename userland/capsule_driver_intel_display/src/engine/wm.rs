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

//! A plane's watermarks (SKL PRM Vol 12, watermark calculations, linear
//! surfaces): for each latency level, the blocks the plane must hold to
//! ride out that latency, by the method fit for its line size, and the
//! lines that is. A level that does not fit the plane's share of the
//! buffer, or needs more than 31 lines, is off, and so is every level
//! above it. Formulas as this driver reads the PRM, unverified there.

/// One level: enable, lines (18:14) and blocks (9:0) of PLANE_WM.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Level {
    pub enabled: bool,
    pub lines: u32,
    pub blocks: u32,
}

impl Level {
    pub fn reg(&self) -> u32 {
        ((self.enabled as u32) << 31) | ((self.lines & 0x1F) << 14) | (self.blocks & 0x3FF)
    }
}

/// The levels for a plane `width` pixels wide at 4 bytes a pixel, on a
/// pipe with line total `htotal` at `pixel_khz`, holding `share` blocks.
pub fn levels(
    latency_us: &[u32; 8],
    width: u32,
    htotal: u32,
    pixel_khz: u32,
    share: u32,
) -> [Level; 8] {
    let mut out = [Level::default(); 8];
    if width == 0 || htotal == 0 || pixel_khz == 0 {
        return out;
    }
    let bytes_per_line = width as u64 * 4;
    let blocks_per_line = bytes_per_line.div_ceil(512);
    for (i, &lat) in latency_us.iter().enumerate() {
        if lat == 0 {
            break;
        }
        let lat = lat as u64;
        // Method 1: the bytes fetched during the latency, in blocks.
        let m1 = (lat * pixel_khz as u64 * 4).div_ceil(512 * 1000);
        // Method 2: the lines scanned during the latency, whole lines.
        let m2 = (lat * pixel_khz as u64).div_ceil(htotal as u64 * 1000) * blocks_per_line;
        let chosen = if bytes_per_line < 512 { m1 } else { m1.min(m2) };
        let blocks = chosen + 1;
        let lines = chosen.div_ceil(blocks_per_line);
        if blocks >= share as u64 || lines > 31 {
            break;
        }
        out[i] = Level { enabled: true, lines: lines as u32, blocks: blocks as u32 };
    }
    out
}
