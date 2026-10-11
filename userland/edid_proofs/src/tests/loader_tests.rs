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

//! The crate reads a base block exactly as the loader's parse does, so the
//! loader can mount it with its EDID lines unchanged.

use crate::build::{base, display, dtd, dtd_1080p, put, seal};
use crate::edid::Edid;
use crate::loader_pick::parse_edid;

fn same(b: &[u8]) {
    let ours = Edid::parse(b).ok().and_then(|e| e.native());
    let theirs = parse_edid(b);
    let ours = ours.map(|n| (n.width, n.height, n.width_mm, n.height_mm));
    let theirs = theirs.map(|e| (e.width, e.height, e.width_mm, e.height_mm));
    assert_eq!(ours, theirs);
}

#[test]
fn the_ordinary_panels_agree() {
    let mut b = base();
    same(&b);
    put(&mut b, 0, &dtd_1080p());
    same(&b);
    let mut b = base();
    put(&mut b, 0, &display(0xFC, b"PANEL\n"));
    put(&mut b, 1, &dtd(138_500, [1920, 160, 48, 32, 1200, 35, 3, 6], (0, 0), 0x1A));
    same(&b);
    let i = dtd(74_250, [1920, 280, 88, 44, 540, 22, 2, 5], (0, 50), 0x9E);
    put(&mut b, 1, &i);
    same(&b);
}

#[test]
fn noise_in_the_base_block_agrees_too() {
    let mut x: u32 = 0x9E37_79B9;
    for _ in 0..20_000 {
        let mut b = base();
        for v in b[20..126].iter_mut() {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            *v = (x >> 24) as u8;
        }
        seal(&mut b);
        same(&b);
    }
}
