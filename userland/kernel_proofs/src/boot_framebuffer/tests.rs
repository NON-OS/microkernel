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

use super::rule::{encode, window, Geometry, INFO_LEN};

#[test]
fn a_page_aligned_frame_is_published_as_it_lies() {
    // 1280x800 at a 5120-byte pitch, its first pixel 0x40 into its page.
    let len = 0x40 + 5120 * 800;
    assert_eq!(window(0xC000_0000, len, 0x40), Some((0xC000_0000, len)));
    assert_eq!(window(0x8000_0000, 4096, 0), Some((0x8000_0000, 4096)));
}

#[test]
fn a_window_no_loader_should_hand_over_is_refused() {
    assert_eq!(window(0, 4096, 0), None, "no address");
    assert_eq!(window(0xC000_0010, 4096, 0), None, "a page base off its page");
    assert_eq!(window(0xC000_0000, 4096, 4096), None, "an offset past the page");
    assert_eq!(window(0xC000_0000, 0x40, 0x40), None, "no pixel after the offset");
    assert_eq!(window(0xC000_0000, 0, 0), None, "an empty frame");
    assert_eq!(window(!0xFFFu64, 0x2000, 0), None, "past the address space");
}

#[test]
fn the_geometry_is_six_little_endian_words_in_order() {
    let g = Geometry {
        width: 1280,
        height: 800,
        stride: 5120,
        format: 3,
        offset: 0x40,
        phys_mm: 286 | (179 << 16),
    };
    let bytes = encode(g);
    assert_eq!(bytes.len(), INFO_LEN);
    let word = |i: usize| u32::from_le_bytes(bytes[i * 4..i * 4 + 4].try_into().unwrap());
    let fields = [1280, 800, 5120, 3, 0x40, 286 | (179 << 16)];
    for (i, want) in fields.into_iter().enumerate() {
        assert_eq!(word(i), want, "word {i}");
    }
}

#[test]
fn an_unknown_panel_size_is_zero() {
    let g = Geometry { width: 1, height: 1, stride: 4, format: 2, offset: 0, phys_mm: 0 };
    assert_eq!(&encode(g)[20..24], &[0, 0, 0, 0]);
}
