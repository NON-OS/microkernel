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

//! The pieces hostile output is built from: control sequences with random
//! parameters and finals, strings, broken UTF-8, wide and combining
//! characters, controls, text and raw noise.

use super::rng::Rng;

const FINALS: &[u8] = b"@ABCDEFGHIJKLMPSTXZ`abcdefghlmnqrstu";

pub fn piece(r: &mut Rng, out: &mut Vec<u8>) {
    let n = |r: &mut Rng, m: u64| (r.next() % m) as usize;
    match n(r, 12) {
        0..=2 => {
            out.extend_from_slice(b"\x1b[");
            if n(r, 3) == 0 {
                out.push(b"?><="[n(r, 4)]);
            }
            for _ in 0..n(r, 6) {
                let v = [0, 1, 2, 5, 7, 25, 38, 48, 1049, 2004, 65535, 999_999][n(r, 12)];
                out.extend_from_slice(v.to_string().as_bytes());
                out.push(if n(r, 4) == 0 { b':' } else { b';' });
            }
            if n(r, 8) == 0 {
                out.push(b" !$\"'"[n(r, 5)]);
            }
            out.push(FINALS[n(r, FINALS.len() as u64)]);
        }
        3 => {
            out.extend_from_slice(b"\x1b]");
            out.extend_from_slice(
                ["0;t", "4;1;?", "8;;x", "52;c;?", "11;?", "133;A", "7;file:///p"][n(r, 7)]
                    .as_bytes(),
            );
            out.extend_from_slice(if n(r, 2) == 0 { b"\x07" } else { b"\x1b\\" });
        }
        4 => out.extend_from_slice(
            ["\x1bP+q544e\x1b\\", "\x1bP$qr\x1b\\", "\x1b_x\x1b\\"][n(r, 3)].as_bytes(),
        ),
        5 => out.extend_from_slice(
            ["\x1b7", "\x1b8", "\x1bM", "\x1bc", "\x1b(0", "\x1b#8"][n(r, 6)].as_bytes(),
        ),
        6 => out.extend_from_slice("漢字👩\u{200d}💻e\u{301}\u{1f3fb}".as_bytes()),
        7 => out.extend_from_slice(&[0xC3, 0xE2, 0x82, 0xF0, 0x9F, 0xED, 0xA0][..n(r, 7) + 1]),
        8 => out.push([b'\n', b'\r', b'\t', 0x08, 0x0E, 0x0F, 0x18, 0x07][n(r, 8)]),
        9 => out.extend_from_slice(b"the quick brown fox "),
        _ => {
            for _ in 0..n(r, 16) {
                out.push((r.next() >> 56) as u8);
            }
        }
    }
}
