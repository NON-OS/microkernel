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

//! A monitor's EDID for the tests, byte by byte from VESA E-EDID 1.4
//! section 3: the header, version 1.4, a 1920x1080 detailed timing in the
//! first descriptor with its image size, the rest dummy descriptors.

/// 1920x1080 at 148.5 MHz (the CTA-861 VIC 16 timing), `mm` in size.
pub fn edid_1080p(mm: (u32, u32)) -> Vec<u8> {
    let mut b = vec![0u8; 128];
    b[..8].copy_from_slice(&[0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0]);
    (b[18], b[19], b[20], b[21], b[22]) = (1, 4, 0xA5, 34, 19);
    b[38..54].fill(0x01);
    let (ha, hb, va, vb) = (1920u32, 280u32, 1080u32, 45u32);
    let d = &mut b[54..72];
    d[..2].copy_from_slice(&14_850u16.to_le_bytes());
    (d[2], d[3], d[4]) = (ha as u8, hb as u8, ((ha >> 8) << 4 | hb >> 8) as u8);
    (d[5], d[6], d[7]) = (va as u8, vb as u8, ((va >> 8) << 4 | vb >> 8) as u8);
    (d[8], d[9], d[10]) = (88, 44, 4 << 4 | 5);
    (d[12], d[13], d[14]) = (mm.0 as u8, mm.1 as u8, ((mm.0 >> 8) << 4 | mm.1 >> 8) as u8);
    d[17] = 0x1E;
    for i in 1..4 {
        b[54 + i * 18 + 3] = 0x10;
    }
    let sum = b[..127].iter().fold(0u8, |a, &v| a.wrapping_add(v));
    b[127] = sum.wrapping_neg();
    b
}
