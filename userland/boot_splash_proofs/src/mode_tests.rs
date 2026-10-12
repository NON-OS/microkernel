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

//! F-60's firmware mode in the record: read when the kernel gives it, none
//! for 0 by 0 or an old 28-byte answer, and used to place the logo, so an
//! off-centre logo (where the margins would mislead) lands where it was.

use crate::build::{bmp, record};
use crate::logo::place::Spot;
use crate::logo::prepare::prepare;
use crate::logo::record::{Record, ON_SCREEN};

fn with_mode(r: [u8; 28], w: u32, h: u32) -> Vec<u8> {
    let mut v = r.to_vec();
    v.extend_from_slice(&w.to_le_bytes());
    v.extend_from_slice(&h.to_le_bytes());
    v
}

#[test]
fn the_mode_is_read_when_given() {
    let f = bmp(4, 2, 24, false);
    let r = record(1, 2, &f, ON_SCREEN, 0);
    assert_eq!(Record::parse(&with_mode(r, 1920, 1080)).unwrap().mode, Some((1920, 1080)));
    assert_eq!(Record::parse(&with_mode(r, 0, 0)).unwrap().mode, None, "0 by 0: none kept");
    assert_eq!(Record::parse(&with_mode(r, 1920, 0)).unwrap().mode, None);
    assert_eq!(Record::parse(&r).unwrap().mode, None, "a kernel before F-60");
    assert_eq!(Record::parse(&with_mode(r, 7, 9)[..30]).unwrap().mode, None, "a cut mode");
}

#[test]
fn an_off_centre_logo_is_placed_by_the_mode() {
    let f = bmp(8, 4, 32, false);
    // At (0, 0) in a 16 by 8 mode, on a canvas half its size.
    let rec = with_mode(record(0, 0, &f, ON_SCREEN, 0), 16, 8);
    let s = prepare(&rec, &f, None, (8, 4)).unwrap();
    assert_eq!(s.spot, Spot { x: 0, y: 0, w: 4, h: 2 });
    // The margins alone would take the mode to be 8 wide and keep it full size.
    let guessed = prepare(&rec[..28], &f, None, (8, 4)).unwrap();
    assert_eq!((guessed.spot.w, guessed.spot.h), (8, 4));
    // A mode handed in by the caller still wins.
    assert_eq!(prepare(&rec, &f, Some((8, 4)), (8, 4)).unwrap().spot.w, 8);
}
