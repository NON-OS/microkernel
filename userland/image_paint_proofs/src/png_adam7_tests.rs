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

//! Adam7: the seven passes rebuild the same image as plain scanlines, with
//! the zlib stream split across IDAT chunks at any byte.
use std::vec::Vec;

use crate::png_build::png;
use crate::png_rows::{decode, grey, pass_rows, H, PASSES, W};

#[test]
fn adam7_equals_the_plain_image_and_idat_can_split_anywhere() {
    let plain = png((W as u32, H as u32, 8, 0, 0), &[], &pass_rows([0, 0, 1, 1]), 1 << 20);
    let adam7: Vec<u8> = PASSES.iter().flat_map(|&p| pass_rows(p)).collect();
    let want: Vec<u32> =
        (0..W * H).map(|i| (grey(i % W, i / W) as u32 * 0x01_0101) | 0xff00_0000).collect();
    assert_eq!(decode(&plain, W * H).unwrap(), want);
    for split in [1, 7, 1 << 20] {
        let file = png((W as u32, H as u32, 8, 0, 1), &[], &adam7, split);
        assert_eq!(decode(&file, W * H).unwrap(), want, "IDAT chunks of {split} bytes");
    }
}
