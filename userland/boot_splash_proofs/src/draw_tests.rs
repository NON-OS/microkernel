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

//! Drawing the prepared logo: copied exactly where its spot is, clipped at
//! every edge, and not one pixel written outside the spot or the surface.

use crate::logo::place::Spot;
use crate::logo::Shown;

const BG: u32 = 0x1234_5678;

fn drawn(spot: Spot, w: u32, h: u32, spx: usize) -> Vec<u32> {
    let pixels = (0..spot.w * spot.h).map(|i| 0xFF00_0000 | i).collect();
    let mut buf = vec![BG; spx * h as usize];
    Shown { spot, pixels }.draw(&mut buf, spx, w, h);
    buf
}

fn expect(spot: Spot, w: u32, h: u32, spx: usize, buf: &[u32]) {
    for y in 0..h as i64 {
        for x in 0..spx as i64 {
            let inside = x < w as i64
                && (spot.x..spot.x + spot.w as i64).contains(&x)
                && (spot.y..spot.y + spot.h as i64).contains(&y);
            let want = if inside {
                0xFF00_0000 | ((y - spot.y) * spot.w as i64 + (x - spot.x)) as u32
            } else {
                BG
            };
            assert_eq!(buf[y as usize * spx + x as usize], want, "{spot:?} at {x},{y}");
        }
    }
}

#[test]
fn every_edge_clips_and_nothing_outside_is_touched() {
    let (w, h, spx) = (9, 7, 12);
    for x in -5..11 {
        for y in -4..9 {
            let spot = Spot { x, y, w: 4, h: 3 };
            expect(spot, w, h, spx, &drawn(spot, w, h, spx));
        }
    }
}

#[test]
fn a_spot_larger_than_the_surface_fills_it_from_the_right_place() {
    let spot = Spot { x: -2, y: -1, w: 20, h: 12 };
    expect(spot, 9, 7, 9, &drawn(spot, 9, 7, 9));
}
