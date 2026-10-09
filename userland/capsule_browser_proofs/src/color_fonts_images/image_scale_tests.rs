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

//! Decoding straight to the display size instead of refusing big images.

use super::jpeg_tests::fixture;
use crate::browser::image::decode::{decode_body, natural};

#[test]
fn a_huge_jpeg_decodes_straight_to_its_display_size() {
    let bytes = fixture("big_prog.jpg");
    assert_eq!(natural(&bytes), Some((4096, 2304)));
    let d = decode_body(&bytes, (400, 225)).expect("decodes scaled");
    assert_eq!((d.w, d.h), (400, 225), "the plan fits the 16:9 hint exactly");
    let at = |x: u32, y: u32| d.px[(y * d.w + x) as usize];
    let quadrants =
        [(100, 56, 0xc81e1e), (300, 56, 0x1ec81e), (100, 170, 0x1e1ec8), (300, 170, 0xdcdc28)];
    for (x, y, want) in quadrants {
        let rgb = [16u32, 8, 0].map(|sh| ((at(x, y) >> sh) & 0xff).abs_diff((want >> sh) & 0xff));
        assert!(
            rgb.iter().all(|&e| e <= 6),
            "({x},{y}) is {:06x}, want {want:06x}",
            at(x, y) & 0xffffff
        );
    }
}
