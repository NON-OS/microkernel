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

//! RX transfers built from r8152's struct rx_desc (six little-endian
//! dwords, the length with CRC in opts1's low 15 bits), walked as
//! rx_bottom walks them, hostile lengths included.

use crate::r8153::rx::{frames, RX_DESC};

/// A descriptor, the frame, a CRC, and the padding to the next 8 bytes.
fn packet(opts1: u32, frame: &[u8]) -> Vec<u8> {
    let mut p = opts1.to_le_bytes().to_vec();
    p.resize(RX_DESC, 0);
    p.extend_from_slice(frame);
    p.extend_from_slice(&[0xc5; 4]);
    p.resize(p.len().next_multiple_of(8), 0xee);
    p
}

fn walk(t: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    frames(t, |f| out.push(f.to_vec()));
    out
}

#[test]
fn frames_come_out_without_their_crc_at_eight_byte_steps() {
    let (a, b) = (vec![0x11; 66], vec![0x22; 60]);
    let mut t = packet(70, &a);
    assert_eq!(t.len(), 96, "24 + 70 rounded up to 8");
    t.extend(packet(64, &b));
    assert_eq!(walk(&t), vec![a.clone(), b]);
    // Bits above RX_LEN_MASK are not length.
    assert_eq!(walk(&packet(0xffff_8000 | 70, &a)), vec![a]);
}

#[test]
fn hostile_lengths_end_the_walk_inside_the_transfer() {
    let f = vec![0x33; 60];
    assert!(walk(&packet(59, &[0; 55])).is_empty(), "under ETH_ZLEN");
    assert!(walk(&packet(0x7fff, &f)).is_empty(), "past the end");
    let t = packet(64, &f);
    assert!(walk(&t[..t.len() - 3]).is_empty(), "CRC not all there");
    assert!(walk(&t[..59]).is_empty(), "shorter than ETH_ZLEN");
    let mut t = packet(64, &f);
    t.extend_from_slice(&[0x40, 0, 0, 0, 0, 0]);
    assert_eq!(walk(&t), vec![f], "a descriptor cut short is not read");
}
