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

//! A seeded display answering capabilities requests: its string 40 bytes
//! long or at CAPS_MAX's edges, cut in fragments of one size; each reply
//! honest or, at one of three rates, with one lie (a length byte at an
//! edge, its checksum, the offset echoed one on, the opcode, the address).

use crate::tests::wire_sweep::rng::Rng;

pub struct Plan {
    pub text: Vec<u8>,
    pub chunk: usize,
    pub lies: Vec<u8>,
}

pub fn plan(r: &mut Rng) -> Plan {
    let text = text([40, 1000, 1023, 1024, 1025, 1100][r.below(6) as usize]);
    let chunk = [1, 7, 31, 32][r.below(4) as usize];
    let rate = [0, 10, 40][r.below(3) as usize];
    let lies = (0..2_600).map(|_| if r.below(100) < rate { 1 + r.below(5) as u8 } else { 0 });
    Plan { text, chunk, lies: lies.collect() }
}

/// A string that parses, `len` bytes long (at least its frame's).
pub fn text(len: usize) -> Vec<u8> {
    let (head, tail) = (&b"(prot(monitor)model("[..], &b")vcp(10 12 60(0F 11) D6(01 04)))"[..]);
    let fill = len.saturating_sub(head.len() + tail.len());
    [head, &vec![b'A'; fill], tail].concat()
}

/// The `nth` reply, to a request for `offset`.
pub fn answer(p: &Plan, offset: u16, nth: usize) -> Vec<u8> {
    let at = (offset as usize).min(p.text.len());
    let data = &p.text[at..(at + p.chunk).min(p.text.len())];
    let [hi, lo] = offset.to_be_bytes();
    let mut b = [&[0x6E, 0x80 | (3 + data.len()) as u8, 0xE3, hi, lo][..], data, &[0]].concat();
    let sum = |b: &mut Vec<u8>| {
        let n = b.len() - 1;
        b[n] = b[..n].iter().fold(0x50, |a, v| a ^ v);
    };
    sum(&mut b);
    match p.lies.get(nth).copied().unwrap_or(0) {
        1 => b[1] = [0x82, 0xA4, 0x23, 0xFF][nth % 4],
        2 => *b.last_mut().unwrap() ^= 0x10,
        3 => {
            b[4] = b[4].wrapping_add(1);
            sum(&mut b);
        }
        4 => (b[2], b[0]) = (0xE2, 0x6E),
        5 => b[0] = 0x6F,
        _ => {}
    }
    b
}
