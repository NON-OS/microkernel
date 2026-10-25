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

//! Payloads as a compositor builds them, two times in three honest for
//! the output's primary, else lying in a count, a size or a length.

use super::kit::{ops, Cursor, Flush, Rect};
use super::rng::Rng;

fn flush(r: &mut Rng, honest: bool, serial: u32, (w, h): (u32, u32)) -> Vec<u8> {
    let n = if honest { 1 + r.below(15) as usize } else { r.below(20) as usize };
    let serial = if honest { serial } else { r.word() };
    let mut f = Flush { serial, len: n.min(15), ..Flush::default() };
    for x in f.rects[..n.min(15)].iter_mut() {
        let fair = Rect { x: 0, y: 0, width: 1 + r.below(w.max(1)), height: 1 + r.below(h.max(1)) };
        *x = if honest {
            fair
        } else {
            Rect { x: r.word(), y: r.word(), width: r.word(), height: r.word() }
        };
    }
    let mut out = vec![0u8; 8 + 16 * 15];
    let k = f.encode(&mut out).unwrap_or(0);
    out.truncate(if honest { k } else { r.below(k as u32 + 9) as usize });
    out
}

fn cursor(r: &mut Rng, honest: bool) -> Vec<u8> {
    let side = if honest { r.below(65) as u16 } else { r.word() as u16 };
    let hot_x = r.below(side.max(1) as u32 + 2) as u16;
    let c = Cursor {
        flags: r.below(4),
        width: side,
        height: side,
        hot_x,
        x: r.word() as i32,
        ..Cursor::default()
    };
    c.encode()[..if honest { 20 } else { r.below(21) as usize }].to_vec()
}

/// A payload for `op` on an output known to show `size` as primary `serial`.
pub fn payload(r: &mut Rng, op: u16, serial: u32, size: (u32, u32)) -> Vec<u8> {
    let honest = r.below(3) != 0;
    match op {
        ops::FLUSH => flush(r, honest, serial, size),
        ops::CURSOR => cursor(r, honest),
        ops::MODE_SET => {
            let b: Vec<u8> = [r.word(), r.word()].iter().flat_map(|w| w.to_le_bytes()).collect();
            b[..if honest { 8 } else { r.below(9) as usize }].to_vec()
        }
        ops::RETIRE => (if honest { serial } else { r.word() }).to_le_bytes().to_vec(),
        _ => (0..r.below(17)).map(|_| r.next() as u8).collect(),
    }
}
