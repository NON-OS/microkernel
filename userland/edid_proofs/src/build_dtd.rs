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

//! Descriptors built from E-EDID 1.4 Tables 3.21 and 3.22.

/// A detailed timing descriptor (Table 3.21). `t` is h active, blank,
/// front porch, sync, then the same four vertically.
pub fn dtd(clock_khz: u32, t: [u32; 8], mm: (u32, u32), flags: u8) -> [u8; 18] {
    let c = (clock_khz / 10) as u16;
    let [ha, hb, hf, hs, va, vb, vf, vs] = t;
    let mut d = [0u8; 18];
    d[..2].copy_from_slice(&c.to_le_bytes());
    d[2] = ha as u8;
    d[3] = hb as u8;
    d[4] = ((ha >> 8) << 4 | (hb >> 8)) as u8;
    d[5] = va as u8;
    d[6] = vb as u8;
    d[7] = ((va >> 8) << 4 | (vb >> 8)) as u8;
    d[8] = hf as u8;
    d[9] = hs as u8;
    d[10] = ((vf & 0xF) << 4 | (vs & 0xF)) as u8;
    d[11] = ((hf >> 8) << 6 | (hs >> 8) << 4 | (vf >> 4) << 2 | (vs >> 4)) as u8;
    d[12] = mm.0 as u8;
    d[13] = mm.1 as u8;
    d[14] = ((mm.0 >> 8) << 4 | (mm.1 >> 8)) as u8;
    d[17] = flags;
    d
}

/// 1920x1080 at 60 Hz, the CTA-861 VIC 16 timing, +H +V digital.
pub fn dtd_1080p() -> [u8; 18] {
    dtd(148_500, [1920, 280, 88, 44, 1080, 45, 4, 5], (531, 299), 0x1E)
}

/// A display descriptor with `tag` and up to 13 body bytes.
pub fn display(tag: u8, body: &[u8]) -> [u8; 18] {
    let mut d = [0u8; 18];
    d[3] = tag;
    d[5..5 + body.len()].copy_from_slice(body);
    d
}
