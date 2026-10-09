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

//! JPEG decodes scaled in the IDCT by 1/2, 1/4 and 1/8.

use nonos_toolkit::image::jpeg::coef::{cost, decode_scaled};

use super::jpeg_tests::{fixture, scaled};

#[test]
fn scaled_decodes_shrink_by_powers_of_two() {
    let c = cost(&fixture("prog_420.jpg")).expect("frame header");
    assert_eq!((c.w, c.h), (61, 45));
    assert!(c.coef[1] < c.coef[0] && c.coef[3] < c.coef[2], "less storage at each step");
    for (shift, size) in [(1, (31, 23)), (2, (16, 12)), (3, (8, 6))] {
        let (w, h, px) = scaled("prog_420.jpg", shift);
        assert_eq!((w, h), size);
        assert_eq!(px.len(), (w * h) as usize);
    }
    let bytes = fixture("prog_420.jpg");
    let r = decode_scaled(&bytes, 0, c.coef[0] - 1, &mut |_, _| {});
    assert!(r.is_err(), "storage past the cap is refused before allocating");
}
