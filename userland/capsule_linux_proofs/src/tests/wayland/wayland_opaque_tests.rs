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

//! An XRGB buffer is taken opaque: every pixel's fourth byte becomes 0xFF,
//! and nothing else in the frame moves.

use crate::opaque::make_opaque;

#[test]
fn every_pixel_gets_full_alpha_and_keeps_its_colour() {
    // 2x2 pixels, 12 bytes a row: 4 bytes of row padding each.
    let mut frame: Vec<u8> = (0..24).map(|i| i as u8).collect();
    let before = frame.clone();
    make_opaque(&mut frame, 2, 2, 12);
    for (i, (now, was)) in frame.iter().zip(&before).enumerate() {
        let (row, col) = (i / 12, i % 12);
        let alpha = col < 8 && col % 4 == 3 && row < 2;
        assert_eq!(*now, if alpha { 0xFF } else { *was }, "byte {i}");
    }
}

#[test]
fn a_frame_shorter_than_its_rows_is_left_whole_past_its_end() {
    // The last row's padding is not in the frame: frame_len may cut there.
    let mut frame = vec![0u8; 12 + 8];
    make_opaque(&mut frame, 2, 2, 12);
    assert_eq!(frame, [0, 0, 0, 0xFF, 0, 0, 0, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0, 0, 0, 0xFF]);
    let mut empty: [u8; 0] = [];
    make_opaque(&mut empty, 4, 4, 16);
}

#[test]
fn rows_past_the_height_are_not_touched() {
    let mut frame = vec![0u8; 3 * 8];
    make_opaque(&mut frame, 2, 1, 8);
    assert_eq!(&frame[..8], &[0, 0, 0, 0xFF, 0, 0, 0, 0xFF]);
    assert!(frame[8..].iter().all(|b| *b == 0));
}
