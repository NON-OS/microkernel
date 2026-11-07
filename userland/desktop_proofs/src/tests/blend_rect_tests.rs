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

//! A colour blended over a box a row at a time is the colour blended pixel
//! by pixel, as blend_rect drew it before, clipped at the buffer's edges
//! the same way; and `over` on a pixel with no alpha is the colour itself,
//! as the full sum gives it, for every alpha and channel.

use nonos_toolkit::paint::mixer::over;
use nonos_toolkit::paint::PaintBuffer;

const W: u32 = 40;
const H: u32 = 30;

/// `over` as it was, with no case for a transparent pixel.
fn over_by_sum(dst: u32, src: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    if sa == 0 {
        return dst;
    }
    if sa == 0xFF {
        return src;
    }
    let da = (dst >> 24) & 0xFF;
    let ia = 255 - sa;
    let oa = sa + (da * ia + 127) / 255;
    let den = oa * 255;
    let mut out = oa << 24;
    for shift in [0, 8, 16] {
        let sc = (src >> shift) & 0xFF;
        let dc = (dst >> shift) & 0xFF;
        out |= ((sc * sa * 255 + dc * da * ia + den / 2) / den) << shift;
    }
    out
}

#[test]
fn over_a_transparent_pixel_is_the_sum_for_every_alpha_and_channel() {
    for sa in 0..=255u32 {
        for sc in 0..=255u32 {
            let src = (sa << 24) | (sc << 16) | ((255 - sc) << 8) | (sc ^ 0x5A);
            for dst in [0, 0x00FF_FFFF, 0x0012_3456] {
                assert_eq!(over(dst, src), over_by_sum(dst, src), "src {src:08x} dst {dst:08x}");
            }
        }
    }
}

#[test]
fn over_a_part_covered_pixel_is_unchanged() {
    for da in [1u32, 2, 77, 128, 254] {
        for sa in [1u32, 3, 90, 160, 254] {
            let (dst, src) = ((da << 24) | 0x0040_80C0, (sa << 24) | 0x00C0_2010);
            assert_eq!(over(dst, src), over_by_sum(dst, src), "da {da} sa {sa}");
        }
    }
}

/// Every pixel's alpha and colour different, some of them transparent.
fn base(len: usize) -> Vec<u32> {
    (0..len as u32)
        .map(|i| {
            let a = [0u32, 0x40, 0xFF, 0x01, 0xC8][i as usize % 5];
            (a << 24) | (i.wrapping_mul(2_654_435_761) >> 8)
        })
        .collect()
}

fn by_pixel(fb: &mut PaintBuffer, (x, y, w, h): (u32, u32, u32, u32), c: u32) {
    for py in y..y.saturating_add(h).min(fb.height) {
        for px in x..x.saturating_add(w).min(fb.width) {
            fb.blend_px(px, py, c);
        }
    }
}

fn drawn(len: usize, stride: u32, f: impl FnOnce(&mut PaintBuffer)) -> Vec<u32> {
    let mut px = base(len);
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: stride, width: W, height: H };
    f(&mut fb);
    px
}

#[test]
fn a_box_blended_by_rows_is_the_box_blended_by_pixels() {
    let boxes = [
        (0, 0, W, H),
        (3, 2, 10, 7),
        (35, 25, 20, 20),
        (0, 29, W, 1),
        (39, 0, 1, H),
        (W, 0, 4, 4),
        (0, H, 4, 4),
        (5, 5, 0, 3),
        (2, 3, u32::MAX, 2),
    ];
    // A full buffer, one with a wider stride, and one cut short mid-row.
    let shapes = [((W * H) as usize, W), ((48 * H) as usize, 48), ((W * 20 + 13) as usize, W)];
    for (len, stride) in shapes {
        for b in boxes {
            for c in [0x8010_E0A0, 0x0300_0000, 0x5A00_0000, 0xFE40_2010, 0x0000_FFFF] {
                let want = drawn(len, stride, |fb| by_pixel(fb, b, c));
                let got = drawn(len, stride, |fb| fb.blend_rect(b.0, b.1, b.2, b.3, c));
                assert!(want == got, "box {b:?} colour {c:08x} stride {stride} len {len}");
            }
        }
    }
}
