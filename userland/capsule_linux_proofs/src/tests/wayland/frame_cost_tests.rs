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

//! What a frame is said to have laid inside the bridge (wayland/frame_cost.rs)
//! is what the lay and the bring-up copy really write.

use crate::damage_lay::lay;
use crate::frame_cost::{box_bytes, corner_bytes};
use crate::swap_copy::{clip, copy};

const W: u32 = 6;
const H: u32 = 5;

fn written(out: &[u8]) -> u64 {
    out.iter().filter(|&&b| b != 0).count() as u64
}

#[test]
fn a_lay_writes_exactly_the_bytes_counted_for_its_box() {
    let stride = W * 4;
    let client = vec![0xAAu8; (stride * H) as usize];
    for x0 in 0..=W {
        for x1 in x0..=W {
            for y0 in 0..=H {
                for y1 in y0..=H {
                    let r = (x0, y0, x1, y1);
                    let mut out = vec![0u8; (stride * H) as usize];
                    assert!(lay(&mut out, (stride, 0, 0), &client, stride, r, None));
                    assert_eq!(written(&out), corner_bytes(r), "box {r:?}");
                }
            }
        }
    }
}

#[test]
fn a_box_turned_inside_out_counts_nothing() {
    assert_eq!(corner_bytes((4, 0, 2, 3)), 0);
    assert_eq!(corner_bytes((0, 4, 3, 2)), 0);
    assert_eq!(box_bytes(0, 9), 0);
}

#[test]
fn a_bring_up_copies_exactly_the_bytes_counted_for_its_clipped_box() {
    let shape = (W, H, W * 4);
    let from = vec![0x55u8; (W * 4 * H) as usize];
    for a in [(0, 0, W, H), (2, 1, 3, 2), (4, 3, 9, 9), (W, 0, 2, 2), (1, 1, 0, 3)] {
        let mut to = vec![0u8; from.len()];
        assert!(copy(&mut to, &from, shape, a));
        let (_, _, bw, bh) = clip(a, (W, H)).unwrap_or_default();
        assert_eq!(written(&to), box_bytes(bw, bh), "box {a:?}");
    }
}

#[test]
fn the_count_saturates_rather_than_wrapping() {
    assert_eq!(box_bytes(u32::MAX, u32::MAX), u64::MAX);
    assert_eq!(box_bytes(1920, 1080), 1920 * 1080 * 4);
}
