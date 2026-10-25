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

//! EDID blobs built byte by byte from the E-EDID 1.4 layout (VESA E-EDID
//! Release A Revision 2, section 3): the base block's header, version,
//! screen size, established timings and one detailed timing in slot 0.

/// Established Timings I and II bits (section 3.8, Table 3.18): byte
/// 0x23 bit 7 is 720x400 at 70 Hz, bit 5 640x480 at 60 Hz; byte 0x24 bit
/// 3 is 1024x768 at 60 Hz. Each is (byte, bit).
pub const EST_720X400: (usize, u8) = (0x23, 7);
pub const EST_640X480: (usize, u8) = (0x23, 5);
pub const EST_1024X768: (usize, u8) = (0x24, 3);

/// A detailed timing descriptor (Table 3.21) for `w` x `h` with the
/// given image size; blanking is a fixed 160 x 30, which no test reads.
pub fn dtd(w: u32, h: u32, mm: (u32, u32)) -> [u8; 18] {
    let (hb, vb) = (160u32, 30u32);
    let mut d = [0u8; 18];
    d[..2].copy_from_slice(&7100u16.to_le_bytes());
    d[2] = w as u8;
    d[3] = hb as u8;
    d[4] = ((w >> 8) << 4 | hb >> 8) as u8;
    d[5] = h as u8;
    d[6] = vb as u8;
    d[7] = ((h >> 8) << 4 | vb >> 8) as u8;
    d[8] = 48;
    d[9] = 32;
    d[10] = 3 << 4 | 6;
    d[12] = mm.0 as u8;
    d[13] = mm.1 as u8;
    d[14] = ((mm.0 >> 8) << 4 | mm.1 >> 8) as u8;
    d[17] = 0x1E;
    d
}

/// A 1.4 base block, digital, `timing` in slot 0, the rest dummy
/// descriptors (tag 0x10), `est` set, `extensions` counted at byte 126
/// (no extension block follows), checksum sealed.
pub fn blob(timing: Option<[u8; 18]>, est: &[(usize, u8)], extensions: u8) -> [u8; 128] {
    let mut b = [0u8; 128];
    b[..8].copy_from_slice(&[0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0]);
    (b[18], b[19], b[20], b[21], b[22]) = (1, 4, 0xA5, 29, 18);
    b[38..54].fill(0x01);
    for &(byte, bit) in est {
        b[byte] |= 1 << bit;
    }
    for i in 0..4 {
        b[54 + i * 18 + 3] = 0x10;
    }
    if let Some(t) = timing {
        b[54..72].copy_from_slice(&t);
    }
    b[126] = extensions;
    let sum = b[..127].iter().fold(0u8, |a, &v| a.wrapping_add(v));
    b[127] = sum.wrapping_neg();
    b
}
