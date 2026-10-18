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

//! A look over real pixels: greyscale gives every pixel equal channels and
//! keeps alpha, and every filter keeps white and grey. What a colour-blind
//! filter is for: cvd_gap_tests.

use crate::damage::Rect;
use crate::display::access_rule::{Access, FILTER_DEUTAN, FILTER_GREYSCALE, FILTER_PROTAN};
use crate::frame_pacer::look::{apply, build};
use crate::sw_blitter::Surface;

pub fn looked(filter: u32, px: &[u32]) -> Vec<u32> {
    let mut px = px.to_vec();
    let look = build(&Access { filter, ..Access::default() }, None).unwrap();
    let byte_len = px.len() as u64 * 4;
    let s = Surface {
        base_va: px.as_mut_ptr() as u64,
        stride: byte_len as u32,
        width: px.len() as u32,
        height: 1,
        byte_len,
    };
    apply(s, Rect { x: 0, y: 0, width: px.len() as u32, height: 1 }, &look);
    px
}

pub fn rgb(p: u32) -> [f32; 3] {
    let [b, g, r, _] = p.to_le_bytes();
    [r, g, b].map(|v| crate::display::srgb_rule::to_linear(v) as f32 / 4095.0)
}

#[test]
fn nothing_chosen_builds_nothing_and_greyscale_is_grey() {
    assert!(build(&Access::default(), None).is_none());
    let out = looked(FILTER_GREYSCALE, &[0x80FF_0000, 0xFF12_9A40]);
    for p in out {
        let [b, g, r, _] = p.to_le_bytes();
        assert!(r == g && g == b, "{p:#x}");
    }
    assert_eq!(looked(FILTER_GREYSCALE, &[0x80FF_0000])[0] >> 24, 0x80, "alpha kept");
}

#[test]
fn every_filter_keeps_white_and_grey() {
    for f in [FILTER_GREYSCALE, FILTER_PROTAN, FILTER_DEUTAN, 4] {
        let out = looked(f, &[0xFFFF_FFFF, 0xFF80_8080, 0xFF00_0000]);
        assert_eq!(out[0], 0xFFFF_FFFF, "filter {f}");
        assert_eq!(out[2], 0xFF00_0000, "filter {f}");
        let [b, g, r, _] = out[1].to_le_bytes();
        assert!([r, g, b].iter().all(|&v| v.abs_diff(0x80) <= 1), "filter {f}: {:#x}", out[1]);
    }
}
