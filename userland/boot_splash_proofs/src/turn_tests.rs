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

//! Quarter turns: the inverse lookup of each turn matches turning the image
//! forward, pixel by pixel, and four quarter turns come back to the start.

use crate::logo::turn::{dims, source};

/// The image `w` by `h` turned clockwise once, built forward.
fn turned_once(img: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let (w, h) = (img[0].len(), img.len());
    (0..w).map(|v| (0..h).map(|u| img[h - 1 - u][v]).collect()).collect()
}

fn shown(img: &[Vec<u32>], t: u8) -> Vec<Vec<u32>> {
    let (w, h) = (img[0].len() as u32, img.len() as u32);
    let (dw, dh) = dims(w, h, t);
    (0..dh)
        .map(|v| {
            (0..dw)
                .map(|u| {
                    let (x, y) = source(u, v, w, h, t);
                    img[y as usize][x as usize]
                })
                .collect()
        })
        .collect()
}

#[test]
fn each_turn_is_that_many_clockwise_quarter_turns() {
    let img: Vec<Vec<u32>> = (0..2).map(|y| (0..3).map(|x| y * 10 + x).collect()).collect();
    let mut forward = img.clone();
    for t in 0..4u8 {
        assert_eq!(shown(&img, t), forward, "{t} quarter turns");
        forward = turned_once(&forward);
    }
    assert_eq!(forward, img, "four quarter turns are none");
    assert_eq!(shown(&img, 1)[0], [10, 0], "the left column becomes the top row");
}

#[test]
fn turned_sides_swap() {
    assert_eq!(dims(300, 200, 0), (300, 200));
    assert_eq!(dims(300, 200, 1), (200, 300));
    assert_eq!(dims(300, 200, 2), (300, 200));
    assert_eq!(dims(300, 200, 3), (200, 300));
}
