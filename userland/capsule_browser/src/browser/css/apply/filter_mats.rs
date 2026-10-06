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

/* The color matrices of Filter Effects 1 for amount `a`, as rows of
 * [r, g, b, offset] plus the alpha scale. */

pub(super) fn grayscale(a: f32) -> [f32; 13] {
    let i = 1. - a;
    mat([
        [0.2126 + 0.7874 * i, 0.7152 - 0.7152 * i, 0.0722 - 0.0722 * i],
        [0.2126 - 0.2126 * i, 0.7152 + 0.2848 * i, 0.0722 - 0.0722 * i],
        [0.2126 - 0.2126 * i, 0.7152 - 0.7152 * i, 0.0722 + 0.9278 * i],
    ])
}

pub(super) fn sepia(a: f32) -> [f32; 13] {
    let i = 1. - a;
    mat([
        [0.393 + 0.607 * i, 0.769 - 0.769 * i, 0.189 - 0.189 * i],
        [0.349 - 0.349 * i, 0.686 + 0.314 * i, 0.168 - 0.168 * i],
        [0.272 - 0.272 * i, 0.534 - 0.534 * i, 0.131 + 0.869 * i],
    ])
}

pub(super) fn saturate(s: f32) -> [f32; 13] {
    mat([
        [0.213 + 0.787 * s, 0.715 - 0.715 * s, 0.072 - 0.072 * s],
        [0.213 - 0.213 * s, 0.715 + 0.285 * s, 0.072 - 0.072 * s],
        [0.213 - 0.213 * s, 0.715 - 0.715 * s, 0.072 + 0.928 * s],
    ])
}

pub(super) fn mat(r: [[f32; 3]; 3]) -> [f32; 13] {
    let mut m = [0.; 13];
    for (i, row) in r.iter().enumerate() {
        m[i * 4..i * 4 + 3].copy_from_slice(row);
    }
    m[12] = 1.;
    m
}
