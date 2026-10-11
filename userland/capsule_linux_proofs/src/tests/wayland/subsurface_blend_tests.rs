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

//! C8u: a subsurface's pixels laid on the window's frame, over it or under
//! it, and clipped to it.

use crate::wayland_sub::blend::{lay, on, Plane};

const OPAQUE_RED: [u8; 4] = [0, 0, 255, 255];
const HALF_BLUE: [u8; 4] = [128, 0, 0, 128];

fn plane(w: u32, h: u32, pad: u32) -> Plane {
    Plane { w, h, stride: w * 4 + pad }
}

fn fill(p: Plane, px: [u8; 4]) -> Vec<u8> {
    let mut v = vec![0xEE; (p.stride * p.h) as usize];
    for r in 0..p.h {
        for c in 0..p.w {
            let at = (r * p.stride + c * 4) as usize;
            v[at..at + 4].copy_from_slice(&px);
        }
    }
    v
}

fn px(v: &[u8], p: Plane, (x, y): (u32, u32)) -> [u8; 4] {
    let at = (y * p.stride + x * 4) as usize;
    [v[at], v[at + 1], v[at + 2], v[at + 3]]
}

#[test]
fn over_keeps_an_opaque_top_and_shows_a_transparent_one_through() {
    assert_eq!(on(OPAQUE_RED, HALF_BLUE), OPAQUE_RED);
    assert_eq!(on([0; 4], OPAQUE_RED), OPAQUE_RED);
    assert_eq!(on(HALF_BLUE, OPAQUE_RED), [128, 0, 127, 255]);
    assert_eq!(on([255; 4], [255; 4]), [255; 4], "never past 255");
}

#[test]
fn a_layer_is_clipped_to_the_frame_and_never_touches_its_padding() {
    let (fp, lp) = (plane(4, 3, 8), plane(3, 3, 4));
    let mut frame = fill(fp, [0, 0, 0, 255]);
    lay(&mut frame, fp, &fill(lp, OPAQUE_RED), lp, (2, -1), true);
    assert_eq!(px(&frame, fp, (2, 0)), OPAQUE_RED);
    assert_eq!(px(&frame, fp, (3, 1)), OPAQUE_RED);
    assert_eq!(px(&frame, fp, (1, 0)), [0, 0, 0, 255]);
    assert_eq!(px(&frame, fp, (2, 2)), [0, 0, 0, 255], "below the layer's last row");
    for r in 0..3 {
        let pad = (r * fp.stride + 16) as usize;
        assert!(frame[pad..pad + 8].iter().all(|b| *b == 0xEE), "row {r}'s padding");
    }
    let before = frame.clone();
    lay(&mut frame, fp, &fill(lp, OPAQUE_RED), lp, (i32::MIN, i32::MAX), true);
    lay(&mut frame, fp, &[1, 2], lp, (0, 0), true);
    assert_eq!(frame, before, "off the frame, or too short: nothing");
}
