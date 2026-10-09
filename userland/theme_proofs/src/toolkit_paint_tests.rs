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

//! OP_COMPONENT_RENDER carries x, y, width and height as any four words a
//! client likes, and the toolkit service paints a panel, a button or a label
//! there into the surface the request names. Whatever the words, painting
//! stays inside the surface and inside the rectangle they name, and takes no
//! arithmetic past the top of a u32: these proofs run with overflow checks
//! on, where a wrapped sum is a panic.

use crate::components::button::{render_button, ButtonStyle};
use crate::components::fill_rect::fill_rect;
use crate::components::label::{render_label, LabelStyle};

const W: u32 = 37;
const H: u32 = 23;
const STRIDE: usize = 40;
const PAINT: u32 = 0xFF00_00FF;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn word(s: &mut u64) -> u32 {
    match xorshift(s) % 5 {
        0 => u32::MAX - (xorshift(s) % 8) as u32,
        1 => W + (xorshift(s) % 4) as u32 - 2,
        2 => (xorshift(s) % 64) as u32,
        3 => 0,
        _ => xorshift(s) as u32,
    }
}

/// Every painted pixel lies in the surface's visible width and inside the
/// rectangle, whose far edges are taken without wrapping.
fn painted_within(buf: &[u32], x: u32, y: u32, w: u32, h: u32) {
    for (i, &p) in buf.iter().enumerate() {
        if p == 0 {
            continue;
        }
        let (px, py) = ((i % STRIDE) as u64, (i / STRIDE) as u64);
        assert!(px < u64::from(W) && py < u64::from(H), "pixel ({px},{py}) off the surface");
        assert!(px >= u64::from(x) && px < u64::from(x) + u64::from(w), "x {px} outside {x}+{w}");
        assert!(py >= u64::from(y) && py < u64::from(y) + u64::from(h), "y {py} outside {y}+{h}");
    }
}

#[test]
fn any_rectangle_paints_inside_the_surface_and_itself() {
    let mut s = 0x5041_494E_0000_0001u64;
    for _ in 0..100_000 {
        let (x, y, w, h) = (word(&mut s), word(&mut s), word(&mut s), word(&mut s));
        let mut buf = vec![0u32; STRIDE * H as usize];
        fill_rect(&mut buf, STRIDE, W, H, (x, y, w, h), PAINT);
        painted_within(&buf, x, y, w, h);
        let mut buf = vec![0u32; STRIDE * H as usize];
        let style = ButtonStyle::default();
        render_button(&mut buf, STRIDE, W, H, x, y, w, h, &[], style);
        painted_within(&buf, x, y, w, h);
        let label: Vec<u8> = (0..xorshift(&mut s) % 8).map(|_| xorshift(&mut s) as u8).collect();
        let mut buf = vec![0u32; STRIDE * H as usize];
        render_button(&mut buf, STRIDE, W, H, x, y, w, h, &label, style);
        let mut buf = vec![0u32; STRIDE * H as usize];
        render_label(&mut buf, STRIDE, W, H, x, y, &label, LabelStyle::default());
        painted_within(&buf, x, y, u32::MAX, u32::MAX);
    }
}

#[test]
fn a_button_at_the_top_of_the_range_paints_nothing_and_does_not_wrap() {
    let mut buf = vec![0u32; STRIDE * H as usize];
    render_button(&mut buf, STRIDE, W, H, u32::MAX - 1, 0, 5, 5, &[], ButtonStyle::default());
    render_button(&mut buf, STRIDE, W, H, 0, u32::MAX - 1, 5, 5, &[], ButtonStyle::default());
    assert!(buf.iter().all(|&p| p == 0));
    // A rectangle running past the edge is cut there, not wrapped back.
    render_button(&mut buf, STRIDE, W, H, 30, 20, u32::MAX, u32::MAX, &[], ButtonStyle::default());
    let painted = buf.iter().filter(|&&p| p != 0).count();
    assert_eq!(painted, ((W - 30) * (H - 20)) as usize);
}
