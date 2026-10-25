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

//! A flush names the primary it draws from and rectangles inside it: one
//! naming any other serial is ESTALE, and a rectangle empty or past the
//! mode on either axis is EINVAL, even from the owner holding the primary;
//! neither moves a pixel into video memory.

use nonos_gfx_serve::Server;
use nonos_gfx_wire::errno::{EINVAL, ESTALE};
use nonos_gfx_wire::{ops, Surface};

use super::silence_tests::{ask, words};
use super::wire_sweep::heap::Heap;
use crate::dispi::DispiMode;
use crate::edid::Panel;
use crate::model::Adapter;

const W: u32 = 64;
const H: u32 = 48;

#[test]
fn a_flush_of_another_primary_or_past_the_mode_is_refused_and_moves_nothing() {
    let mut a = Adapter::new();
    let b = a.bga(Panel::default(), DispiMode { width: W, height: H, stride: W * 4 });
    let mut s = Server::new(b, Heap::new(64));
    let sf = Surface::decode(&ask(&mut s, ops::SURFACE, &[]).1).expect("a primary");
    let base = s.slots[0].current.expect("current").region.base;
    s.platform.memory.get_mut(&base).expect("its memory").fill(0x44);
    for serial in [0, sf.serial + 1, u32::MAX].into_iter().filter(|&n| n != sf.serial) {
        let flush = words(&[serial, 1, 0, 0, W, H]);
        assert_eq!(ask(&mut s, ops::FLUSH, &flush).0, ESTALE, "serial {serial}");
    }
    let rects = [(0, 0, W + 1, H), (0, 0, W, H + 1), (W, 0, 1, 1), (0, H, 1, 1), (0, 0, 0, H)];
    for (x, y, w, h) in rects.into_iter().chain([(u32::MAX, 0, 2, 1)]) {
        let flush = words(&[sf.serial, 1, x, y, w, h]);
        assert_eq!(ask(&mut s, ops::FLUSH, &flush).0, EINVAL, "{x},{y} {w}x{h}");
    }
    assert!(a.vram.iter().all(|&v| v == 0), "a refused flush moved a pixel");
    assert_eq!(ask(&mut s, ops::FLUSH, &words(&[sf.serial, 1, 0, 0, W, H])).0, 0);
    assert!(a.vram[..(W * H * 4) as usize].iter().all(|&v| v == 0x44), "its own flush");
}
