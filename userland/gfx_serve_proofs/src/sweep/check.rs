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

//! An op a driver answered 0: its payload, as the test reads it, was well
//! formed, and its body reads back as the wire defines.

use super::kit::{ops, Caps, Modes, State, Surface};
use super::read::{half, word};

/// The output's primary as the test last saw it: serial, then size.
pub type Known = (u32, (u32, u32));

fn flush_fits(p: &[u8], known: &Known) -> bool {
    let k = word(p, 4).unwrap_or(0) as usize;
    let rect = |i: usize, j: usize| word(p, 8 + 16 * i + 4 * j).map_or(0, u64::from);
    let inside = |i: usize| {
        let (w, h) = (known.1 .0 as u64, known.1 .1 as u64);
        rect(i, 2) > 0
            && rect(i, 3) > 0
            && rect(i, 0) + rect(i, 2) <= w
            && rect(i, 1) + rect(i, 3) <= h
    };
    (1..=15).contains(&k)
        && p.len() >= 8 + 16 * k
        && word(p, 0) == Some(known.0)
        && (0..k).all(inside)
}

fn cursor_fits(p: &[u8]) -> bool {
    let h = |i: usize| half(p, 12 + 2 * i).unwrap_or(0);
    p.len() >= 20 && h(2) <= 64 && h(3) <= 64 && h(0) < h(2).max(1) && h(1) < h(3).max(1)
}

/// Whether the answer named a primary now shown.
pub fn check(op: u16, p: &[u8], body: &[u8], known: &mut Known) -> bool {
    let reads = matches!(op, ops::CAPS | ops::MODES | ops::STATE | ops::SURFACE | ops::MODE_SET);
    assert!(reads || body.is_empty(), "op {op:#x} answered with a body");
    match op {
        ops::CAPS => assert!(Caps::decode(body).is_ok_and(|c| c.outputs > 0)),
        ops::MODES => assert!(Modes::decode(body).is_ok()),
        ops::STATE => assert!(State::decode(body).is_ok()),
        ops::SURFACE | ops::MODE_SET => {
            let s = Surface::decode(body).expect("a surface");
            if op == ops::MODE_SET {
                assert_eq!(Some((s.width, s.height)), word(p, 0).zip(word(p, 4)), "the mode asked");
            }
            *known = (s.serial, (s.width, s.height));
            return true;
        }
        ops::FLUSH => assert!(flush_fits(p, known), "a flush taken only inside the primary shown"),
        ops::CURSOR => assert!(cursor_fits(p), "a cursor taken only in its bounds"),
        ops::RETIRE => assert!(p.len() >= 4),
        ops::HEALTHCHECK => {}
        _ => panic!("an op the wire does not define answered 0: {op:#x}"),
    }
    false
}
