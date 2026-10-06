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

//! The framebuffer stays reachable through the CR3 switch wherever the
//! firmware put it in the low half, including the 64 bit BARs of newer Intel
//! iGPUs and of AMD and NVIDIA cards with resizable BAR, and the low window
//! is not mapped twice.

use crate::paging::constants::IDENTITY_LOW_BYTES;
use crate::paging::fb_window::{framebuffer_identity_reachable, framebuffer_tail, IDENTITY_FB_LIMIT};

const FRAME_1080P: u64 = 7680 * 1080;

#[test]
fn a_framebuffer_under_4_gib_needs_no_extra_mapping() {
    assert_eq!(framebuffer_tail(0x8000_0000, FRAME_1080P), None);
    assert!(framebuffer_identity_reachable(0x8000_0000, FRAME_1080P));
    assert_eq!(framebuffer_tail(0xC000_0000, 4096 * 4 * 1600), None);
}

#[test]
fn a_64_bit_bar_framebuffer_is_mapped_whole() {
    for base in [0x40_0000_0000u64, 0x60_0000_0000, 0xFC_0000_0000, 0x7E00_0000_0000] {
        let (start, len) = framebuffer_tail(base, FRAME_1080P).unwrap();
        assert_eq!(start, base);
        assert!(len >= FRAME_1080P && len % 4096 == 0);
        assert!(framebuffer_identity_reachable(base, FRAME_1080P));
    }
}

#[test]
fn a_frame_straddling_the_window_maps_only_its_tail() {
    let base = IDENTITY_LOW_BYTES - 0x1000;
    let (start, len) = framebuffer_tail(base, 0x3000).unwrap();
    assert_eq!((start, len), (IDENTITY_LOW_BYTES, 0x2000));
}

#[test]
fn an_unaligned_base_rounds_to_whole_pages() {
    let (start, len) = framebuffer_tail(0x40_0000_0100, 0x1000).unwrap();
    assert_eq!((start, len), (0x40_0000_0000, 0x2000));
}

#[test]
fn past_the_low_half_nothing_is_mapped_and_nothing_is_written() {
    let base = IDENTITY_FB_LIMIT;
    assert_eq!(framebuffer_tail(base, FRAME_1080P), None);
    assert!(!framebuffer_identity_reachable(base, FRAME_1080P));
    assert!(!framebuffer_identity_reachable(u64::MAX - 10, FRAME_1080P));
    assert!(!framebuffer_identity_reachable(0, FRAME_1080P));
}
