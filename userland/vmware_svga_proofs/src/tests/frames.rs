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

//! Frames: a rectangle copied into the framebuffer at the device's pitch,
//! the host asked to redraw it, and the FIFO wrapping under a long session.

use super::util::up;
use crate::bench::Bench;
use crate::model::with_svga;
use crate::server::handlers::{flush, transfer};

pub fn body(words: &[u32], offset: Option<u64>) -> Vec<u8> {
    let mut b: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    if let Some(o) = offset {
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&o.to_le_bytes());
    }
    b
}

#[test]
fn a_rectangle_lands_at_the_framebuffer_pitch() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |s| s.pad = 64);
    let px = |x: u64, y: u64| (d.surface.va + y * 4096 + x * 4) as *mut u32;
    // SAFETY: inside the 1024x768 surface.
    unsafe { px(10, 20).write(0xAABBCCDD) };
    assert_eq!(transfer(&mut d, &body(&[1, 10, 20, 5, 5], Some(20 * 4096 + 40))), 0);
    let at = 20 * (4096 + 64) + 40;
    assert_eq!(&bench.vram[at..at + 4], &0xAABBCCDDu32.to_le_bytes());
    assert_eq!(transfer(&mut d, &body(&[1, 10, 20, 5, 5], Some(0))), -22, "offset not the rect's");
    assert_eq!(transfer(&mut d, &body(&[1, 1020, 0, 5, 5], Some(4080))), -22, "past the edge");
    assert_eq!(transfer(&mut d, &body(&[2, 0, 0, 5, 5], Some(0))), -22, "no such resource");
}

#[test]
fn a_flush_is_an_update_the_host_runs_even_across_the_wrap() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    for n in 0..3000u32 {
        let (x, y) = (n % 1000, n % 700);
        assert_eq!(flush(&mut d, &body(&[1, x, y, 24, 68], None)), 0, "flush {n}");
    }
    let updates = with_svga(|s| std::mem::take(&mut s.updates));
    assert_eq!(updates.len(), 3000);
    assert_eq!(updates[2999], (2999 % 1000, 2999 % 700, 24, 68));
}
