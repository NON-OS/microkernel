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

//! The logo resampled to its spot: each spot pixel shows the turned logo's
//! pixel whose centre lies under its own (nearest neighbour, so a logo at
//! its own size comes out pixel for pixel).

use alloc::vec::Vec;

use super::bmp::Bmp;
use super::place::Spot;
use super::turn;

pub fn resample(b: &Bmp, turns: u8, spot: Spot) -> Vec<u32> {
    let (dw, dh) = turn::dims(b.width, b.height, turns);
    let (sw, sh) = (spot.w as u64, spot.h as u64);
    let mut pixels = Vec::with_capacity(spot.w as usize * spot.h as usize);
    for j in 0..sh {
        let v = ((2 * j + 1) * dh as u64 / (2 * sh)) as u32;
        for i in 0..sw {
            let u = ((2 * i + 1) * dw as u64 / (2 * sw)) as u32;
            let (sx, sy) = turn::source(u, v, b.width, b.height, turns);
            pixels.push(b.at(sx, sy));
        }
    }
    pixels
}
