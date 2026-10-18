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

//! flush: the damaged rectangle copied from the surface into the
//! framebuffer at the same place, and nothing outside it; transfer: held to
//! the place flush will read, and drawing nothing itself.

use super::frames::{ask, flush_body, transfer_body};
use super::model::{drawn, paint, served, take, Served, COMPOSITOR};
use crate::adapter::GUARD;
use crate::constants::CLEAR_COLOR;
use crate::serve::{E_INVAL, OP_FLUSH, OP_TRANSFER_TO_HOST};

const W: usize = 64;
const H: usize = 48;

/// Every framebuffer pixel: what `inside` pixels were drawn, the clear
/// colour elsewhere, and the guard past the mapping untouched.
fn shows(s: &Served, inside: impl Fn(usize, usize) -> bool) {
    for (y, x) in (0..H).flat_map(|y| (0..W).map(move |x| (y, x))) {
        let i = y * W + x;
        let want = if inside(x, y) { drawn(i) } else { CLEAR_COLOR };
        assert_eq!(s.a.fb.wrote32(i * 4), want, "pixel ({x}, {y})");
    }
    assert_eq!(s.a.fb.wrote32(W * H * 4), GUARD, "written past the framebuffer mapping");
}

#[test]
fn a_flush_copies_the_damaged_rectangle_and_nothing_outside_it() {
    let mut s = served(W as u16, H as u16);
    take(&mut s);
    paint(&s);
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_FLUSH, &flush_body(5, 7, 9, 4)).0, 0);
    shows(&s, |x, y| (5..14).contains(&x) && (7..11).contains(&y));
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_FLUSH, &flush_body(0, 0, 64, 48)).0, 0);
    shows(&s, |_, _| true);
}

#[test]
fn a_transfer_names_where_flush_will_read_and_draws_nothing_itself() {
    let mut s = served(W as u16, H as u16);
    take(&mut s);
    paint(&s);
    let at = ((2 * W + 3) * 4) as u64;
    let transfer = |s: &mut Served, offset| {
        ask(&mut s.d, COMPOSITOR, OP_TRANSFER_TO_HOST, &transfer_body(3, 2, 4, 5, offset)).0
    };
    assert_eq!(transfer(&mut s, at), 0);
    for offset in [0, at - 4, at + 4, at + (W * 4) as u64, u64::MAX] {
        assert_eq!(transfer(&mut s, offset), E_INVAL, "offset {offset} is not where (3, 2) is");
    }
    shows(&s, |_, _| false);
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_FLUSH, &flush_body(3, 2, 4, 5)).0, 0);
    shows(&s, |x, y| (3..7).contains(&x) && (2..7).contains(&y));
}
