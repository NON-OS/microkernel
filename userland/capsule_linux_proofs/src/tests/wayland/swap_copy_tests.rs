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

//! A buffer taken again brought up to the picture (wayland/swap_copy.rs):
//! the stale box less what the draw covers, copied from the buffer shown.

use crate::swap_copy::{clip, copy, outside, Area};

fn has(a: Area, (x, y): (u32, u32)) -> bool {
    x >= a.0 && y >= a.1 && x < a.0 + a.2 && y < a.1 + a.3
}

#[test]
fn every_stale_pixel_outside_the_cover_is_in_exactly_one_part_and_no_other_pixel_is() {
    // Every stale box and cover on a 7 by 6 grid, each corner and size.
    let boxes = || {
        (0..7u32).flat_map(|x| {
            (0..6u32).flat_map(move |y| {
                (0..=7 - x).flat_map(move |w| (0..=6 - y).map(move |h| (x, y, w, h)))
            })
        })
    };
    let covers: Vec<Area> = boxes().step_by(7).collect();
    for stale in boxes().step_by(3) {
        for &cover in &covers {
            let parts = outside(stale, cover);
            for x in 0..8 {
                for y in 0..7 {
                    let want = has(stale, (x, y)) && !has(cover, (x, y));
                    let n = parts.iter().flatten().filter(|p| has(**p, (x, y))).count();
                    assert_eq!(n, usize::from(want), "{stale:?} less {cover:?} at {x},{y}");
                }
            }
            assert!(parts.iter().flatten().all(|p| p.2 > 0 && p.3 > 0), "no empty part");
        }
    }
}

#[test]
fn a_cover_over_the_whole_stale_box_copies_nothing() {
    assert_eq!(outside((10, 10, 50, 40), (0, 0, 100, 100)), [None; 4]);
    assert_eq!(outside((10, 10, 50, 40), (10, 10, 50, 40)), [None; 4]);
    assert_eq!(outside((0, 0, 0, 9), (0, 0, 1, 1)), [None; 4], "an empty stale box");
    // A cover elsewhere, or empty, leaves the stale box whole.
    assert_eq!(outside((10, 10, 5, 5), (40, 40, 5, 5))[0], Some((10, 10, 5, 5)));
    assert_eq!(outside((10, 10, 5, 5), (10, 10, 0, 5))[0], Some((10, 10, 5, 5)));
}

#[test]
fn a_box_is_cut_to_the_buffer() {
    assert_eq!(clip((0, 0, 10, 10), (4, 3)), Some((0, 0, 4, 3)));
    assert_eq!(clip((3, 2, 10, 10), (4, 3)), Some((3, 2, 1, 1)));
    assert_eq!(clip((4, 0, 1, 1), (4, 3)), None, "past the right edge");
    assert_eq!(clip((0, 0, 0, 1), (4, 3)), None, "empty");
    assert_eq!(clip((u32::MAX, 0, 2, 2), (4, 3)), None, "no wrap");
}

#[test]
fn a_copy_moves_the_box_alone_row_for_row_at_the_stride() {
    // 5 by 4 at a stride of 6 pixels: the padding is never written.
    let (w, h, stride) = (5u32, 4u32, 24u32);
    let from: Vec<u8> = (0..stride * h).map(|i| (i % 251) as u8 + 1).collect();
    let mut to = vec![0u8; from.len()];
    assert!(copy(&mut to, &from, (w, h, stride), (1, 1, 3, 2)));
    for y in 0..h {
        for x in 0..stride / 4 {
            let at = (y * stride + x * 4) as usize;
            let inside = (1..4).contains(&x) && (1..3).contains(&y);
            let want = if inside { &from[at..at + 4] } else { &[0u8; 4][..] };
            assert_eq!(&to[at..at + 4], want, "pixel {x},{y}");
        }
    }
    // A box past the buffer is cut to it; slices too short say so.
    let mut to = vec![0u8; from.len()];
    assert!(copy(&mut to, &from, (w, h, stride), (3, 2, 100, 100)));
    assert_eq!(
        &to[(3 * stride + 4 * 4) as usize..][..4],
        &from[(3 * stride + 4 * 4) as usize..][..4]
    );
    let mut short = vec![0u8; 30];
    assert!(!copy(&mut short, &from, (w, h, stride), (0, 0, 5, 4)));
}
