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

//! The three colour-vision filters in linear RGB. `simulate` is what a
//! dichromat sees (`cvd_rule`); `correct` adds back what it loses, shifted
//! into the channels left: for protan and deutan the lost red-green
//! difference goes into green and blue, for tritan the lost blue-yellow one
//! into red and green, after the error matrices Fidaner, Lin and Ozguven
//! (2005) give (from memory, unverified). Colours a dichromat confuses then
//! differ in what they can still see; greys and the anchors stay as they are.

use super::cvd_rule::{inverse, mul, projection, M3, RGB_TO_LMS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cvd {
    Protan,
    Deutan,
    Tritan,
}

const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];
const RED: [f32; 3] = [1.0, 0.0, 0.0];

pub fn simulate(kind: Cvd) -> Option<M3> {
    let p = match kind {
        Cvd::Protan => projection(0, [WHITE, BLUE])?,
        Cvd::Deutan => projection(1, [WHITE, BLUE])?,
        Cvd::Tritan => projection(2, [WHITE, RED])?,
    };
    Some(mul(&inverse(&RGB_TO_LMS)?, &mul(&p, &RGB_TO_LMS)))
}

/// I + E (I - S): the colour plus its lost part, moved where it is seen.
pub fn correct(kind: Cvd) -> Option<M3> {
    let s = simulate(kind)?;
    let e: M3 = match kind {
        Cvd::Protan | Cvd::Deutan => [[0.0, 0.0, 0.0], [0.7, 1.0, 0.0], [0.7, 0.0, 1.0]],
        Cvd::Tritan => [[1.0, 0.0, 0.7], [0.0, 1.0, 0.7], [0.0, 0.0, 0.0]],
    };
    let mut lost = [[0.0; 3]; 3];
    for (i, row) in lost.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = if i == j { 1.0 } else { 0.0 } - s[i][j];
        }
    }
    let mut m = mul(&e, &lost);
    for (i, row) in m.iter_mut().enumerate() {
        row[i] += 1.0;
    }
    Some(m)
}
