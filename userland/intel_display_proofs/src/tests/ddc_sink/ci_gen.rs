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

//! Seeded DDC/CI replies: a Get VCP reply honest or with one lie (its
//! address, a length byte at an edge, opcode, result, the code echoed, the
//! checksum), sealed again half the time so a field's lie is not just a
//! checksum's; or the null message, now and then with a bad checksum.

use crate::tests::wire_sweep::rng::Rng;

/// Length bytes at the edges of the two G-42 takes: the null 0x80 and a
/// VCP reply's 0x88, their neighbours, a fragment's 0xA3, and the far ends.
pub const LENGTHS: [u8; 9] = [0x00, 0x7F, 0x80, 0x81, 0x87, 0x89, 0xA3, 0xA4, 0xFF];

fn seal(b: &mut [u8], at: usize) {
    b[at] = b[..at].iter().fold(0x50, |a, v| a ^ v);
}

pub fn vcp_reply(r: &mut Rng, code: u8) -> Vec<u8> {
    let mut b: Vec<u8> = (0..11).map(|_| r.next() as u8).collect();
    if r.below(8) == 0 {
        (b[0], b[1]) = (0x6E, 0x80);
        seal(&mut b, 2);
        b[2] ^= if r.below(3) == 0 { 1 << r.below(8) } else { 0 };
        return b;
    }
    (b[0], b[1], b[2], b[3], b[4], b[5]) = (0x6E, 0x88, 0x02, 0, code, 0);
    seal(&mut b, 10);
    match r.below(9) {
        0 => b[0] = r.next() as u8,
        1 => b[1] = LENGTHS[r.below(9) as usize],
        2 => b[2] = [0x00, 0x01, 0x03, 0xE3][r.below(4) as usize],
        3 => b[3] = [1, 2, 0xFF][r.below(3) as usize],
        4 => b[4] ^= 1 << r.below(8),
        5 => b[10] ^= 1 << r.below(8),
        _ => return b,
    }
    if r.below(2) == 0 {
        seal(&mut b, 10);
    }
    b
}
