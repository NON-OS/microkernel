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

//! A small grey image as PNG scanlines, Up-filtered per pass, for the
//! streaming decoder's tests.
use nonos_toolkit::image::png::decoder::decode_png_argb8888;
use nonos_toolkit::image::types::DecodeError;
use std::vec::Vec;

pub const PASSES: [[usize; 4]; 7] = [
    [0, 0, 8, 8],
    [4, 0, 8, 8],
    [0, 4, 4, 8],
    [2, 0, 4, 4],
    [0, 2, 2, 4],
    [1, 0, 2, 2],
    [0, 1, 1, 2],
];
pub const W: usize = 11;
pub const H: usize = 9;

pub fn grey(x: usize, y: usize) -> u8 {
    (x * 29 + y * 71 + x * y * 5) as u8
}

/* Scanlines of the pixels at (x0 + i*dx, y0 + j*dy), Up-filtered per pass. */
pub fn pass_rows(p: [usize; 4]) -> Vec<u8> {
    let (mut raw, mut prev) = (Vec::new(), Vec::new());
    for y in (p[1]..H).step_by(p[3]) {
        let row: Vec<u8> = (p[0]..W).step_by(p[2]).map(|x| grey(x, y)).collect();
        raw.push(2);
        raw.extend(row.iter().enumerate().map(|(i, v)| v.wrapping_sub(*prev.get(i).unwrap_or(&0))));
        prev = row;
    }
    raw
}

pub fn decode(file: &[u8], n: usize) -> Result<Vec<u32>, DecodeError> {
    let mut out = std::vec![0u32; n];
    decode_png_argb8888(file, &mut out).map(|_| out)
}
