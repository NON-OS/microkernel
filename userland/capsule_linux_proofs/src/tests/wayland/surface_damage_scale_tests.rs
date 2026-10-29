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

//! A buffer shown at another size has only the box it said changed
//! resampled, and that box is exactly the shown pixels whose sample falls
//! in the changed box, as the whole-frame resampler takes them.

use crate::damage_scale::{resample_box, shown_box};
use crate::wayland_frame::resample::resample;

const SIZES: [((u32, u32), (u32, u32)); 4] =
    [((819, 617), (1023, 771)), ((10, 7), (13, 9)), ((16, 16), (16, 16)), ((20, 12), (10, 6))];

fn sample(d: u32, f: u32, t: u32) -> u32 {
    (((2 * d + 1) * f) / (2 * t)).min(f - 1)
}

#[test]
fn the_shown_box_is_every_pixel_sampled_from_the_changed_box() {
    for (from, to) in SIZES {
        for b in [(0, 0, 1, 1), (3, 2, 7, 5), (0, 0, from.0, from.1), (from.0 - 1, 1, from.0, 2)] {
            let s = shown_box(b, from, to);
            for dx in 0..to.0 {
                let inside = (b.0..b.2).contains(&sample(dx, from.0, to.0));
                assert_eq!(inside, (s.0..s.2).contains(&dx), "x {dx} of {from:?} at {to:?}");
            }
            for dy in 0..to.1 {
                let inside = (b.1..b.3).contains(&sample(dy, from.1, to.1));
                assert_eq!(inside, (s.1..s.3).contains(&dy), "y {dy} of {from:?} at {to:?}");
            }
        }
    }
}

#[test]
fn the_box_resampled_alone_matches_the_whole_frame_resampled() {
    let ((fw, fh), (tw, th)) = ((10u32, 7u32), (13u32, 9u32));
    let stride = fw * 4 + 8;
    let src: Vec<u8> = (0..stride * fh).map(|i| (i * 7 % 251) as u8).collect();
    let mut whole = vec![0u8; (tw * th * 4) as usize];
    assert!(resample(&src, (fw, fh, stride), &mut whole, (tw, th)));
    let b = (2, 1, 6, 5);
    let s = shown_box(b, (fw, fh), (tw, th));
    let rows = &src[(b.1 * stride) as usize..(b.3 * stride) as usize];
    let part = resample_box(rows, (stride, b.1), (fw, fh), (tw, th), s).expect("room");
    for dy in s.1..s.3 {
        for dx in s.0..s.2 {
            let w = ((dy * tw + dx) * 4) as usize;
            let p = (((dy - s.1) * tw + dx) * 4) as usize;
            assert_eq!(&part[p..p + 4], &whole[w..w + 4], "({dx}, {dy})");
        }
    }
}
