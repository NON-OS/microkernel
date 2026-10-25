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

//! Frames through the wire: a flushed rectangle copied into the framebuffer
//! at the device's pitch, the host asked to redraw exactly it, the FIFO
//! wrapping under a long session, and a rectangle outside the primary
//! refused before anything is copied or sent.

use super::ask::{ask, primed, words};
use super::util::up;
use crate::bench::Bench;
use crate::model::with_svga;
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::ops;

#[test]
fn a_flushed_rectangle_lands_at_the_framebuffer_pitch_and_is_redrawn() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |s| s.pad = 64);
    let sf = primed(&mut d.server, 0xAABB_CCDD);
    assert_eq!(ask(&mut d.server, ops::FLUSH, &words(&[sf.serial, 1, 10, 20, 5, 5])).0, 0);
    let pitch = 1024 * 4 + 64;
    let at = |x: usize, y: usize| y * pitch + x * 4;
    let px = |o: usize| u32::from_le_bytes(bench.vram[o..o + 4].try_into().unwrap());
    assert_eq!((px(at(10, 20)), px(at(14, 24))), (0xAABB_CCDD, 0xAABB_CCDD), "the corners");
    assert_eq!((px(at(9, 20)), px(at(15, 24)), px(at(10, 25))), (0, 0, 0), "nothing outside");
    let updates = with_svga(|s| std::mem::take(&mut s.updates));
    assert_eq!(updates, [(10, 20, 5, 5)], "the host redraws the rectangle, once");
}

#[test]
fn a_rectangle_outside_the_primary_is_refused_whole() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let sf = primed(&mut d.server, 0xFFFF_FFFF);
    let flush = |r: [u32; 4]| words(&[sf.serial, 1, r[0], r[1], r[2], r[3]]);
    assert_eq!(ask(&mut d.server, ops::FLUSH, &flush([1020, 0, 5, 5])).0, EINVAL, "past the edge");
    assert_eq!(ask(&mut d.server, ops::FLUSH, &flush([0, 0, 0, 5])).0, EINVAL, "empty");
    assert!(bench.vram.iter().all(|&b| b == 0), "nothing copied");
    assert!(with_svga(|s| s.updates.is_empty()), "nothing sent");
}

#[test]
fn a_flush_is_an_update_the_host_runs_even_across_the_wrap() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let sf = primed(&mut d.server, 1);
    for n in 0..3000u32 {
        let (x, y) = (n % 1000, n % 700);
        let st = ask(&mut d.server, ops::FLUSH, &words(&[sf.serial, 1, x, y, 24, 68])).0;
        assert_eq!(st, 0, "flush {n}");
    }
    let updates = with_svga(|s| std::mem::take(&mut s.updates));
    assert_eq!(updates.len(), 3000);
    assert_eq!(updates[2999], (2999 % 1000, 2999 % 700, 24, 68));
}
