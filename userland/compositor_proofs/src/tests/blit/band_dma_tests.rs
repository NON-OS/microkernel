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
//! The band's no-IOMMU underline: while device DMA is not confined by
//! hardware, the band's bottom two rows are red across its width; the marks
//! above stay as they were, nothing outside the band changes, and only the
//! damaged part is drawn. Without the flag the band is as before.

use crate::damage::Rect;
use crate::frame_pacer::indicator_band::*;
use crate::state::indicator::Indicators;
use crate::sw_blitter::Surface;

const W: u32 = 120;
const H: u32 = 40;
const WINDOW: u32 = 0xFFE0_E0E0;
const RED: u32 = 0xFFE4_574F;

fn drawn(clip: Rect, dma: bool) -> Vec<u32> {
    let mut px = vec![WINDOW; (W * H) as usize];
    let byte_len = px.len() as u64 * 4;
    let s =
        Surface { base_va: px.as_mut_ptr() as u64, stride: W * 4, width: W, height: H, byte_len };
    draw(s, clip, &Indicators::default(), false, false, dma);
    px
}

const ALL: Rect = Rect { x: 0, y: 0, width: W, height: H };

#[test]
fn red_under_the_marks_while_unconfined() {
    assert_eq!(DMA_UNCONFINED, RED);
    let px = drawn(ALL, true);
    let at = |x: u32, y: u32| px[(y * W + x) as usize];
    let b = band(W, H);
    for x in b.x..W {
        assert_eq!((at(x, 14), at(x, 15)), (RED, RED), "column {x}");
        assert_ne!(at(x, 13), RED, "column {x}: only two rows");
    }
    assert_eq!((at(b.x - 1, 15), at(b.x, 16)), (WINDOW, WINDOW), "nothing outside the band");
    assert_eq!(at(b.x + 8, 5), MARK_OFF, "the marks are kept");
}

#[test]
fn confined_the_band_is_as_before() {
    let px = drawn(ALL, false);
    let b = band(W, H);
    assert!(px.iter().all(|p| *p != RED));
    assert_eq!(px[(15 * W + b.x) as usize], BAND_ARGB);
}

#[test]
fn only_the_damaged_part_is_drawn() {
    let b = band(W, H);
    let clip = Rect { x: b.x, y: 15, width: 3, height: 1 };
    let px = drawn(clip, true);
    let red: Vec<usize> = (0..px.len()).filter(|i| px[*i] == RED).collect();
    let want: Vec<usize> = (0..3).map(|d| (15 * W + b.x + d) as usize).collect();
    assert_eq!(red, want);
}
