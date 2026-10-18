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
//! What the tests share: the owner's primary filled and flushed, what the
//! host shows, and whether the host still reads only pages the driver holds.

use nonos_gfx_wire::{ops, Surface};
use nonos_libc::dma_spans;

use super::ask::{ask, surface, words};
use super::rig::Rig;
use crate::constants::GPU_CFG_EVENTS_READ;
use crate::tests::model::DEVICE_AT;

/// The device configuration's events-read word, in the modern window.
pub const EVENTS_READ: usize = DEVICE_AT + GPU_CFG_EVENTS_READ;

/// SURFACE for `o`, its primary filled with `px` and flushed whole.
pub fn draw(r: &mut Rig, o: u16, px: u32) -> Surface {
    let s = surface(r, o);
    fill(r, o, px);
    let all = words(&[s.serial, 1, 0, 0, s.width, s.height]);
    assert_eq!(ask(r, ops::FLUSH, o, &all).0, 0, "flushed");
    s
}

pub fn fill(r: &mut Rig, o: u16, px: u32) {
    let base = r.d.server.slots[o as usize].current.expect("a primary").region.base;
    let m = r.d.server.platform.memory.get_mut(&base).expect("its memory");
    m.chunks_exact_mut(4).for_each(|c| c.copy_from_slice(&px.to_le_bytes()));
}

/// The pixels scanout `s` shows, if any.
pub fn shown(r: &Rig, s: usize) -> Option<Vec<u32>> {
    r.host.borrow().shown(s).map(|p| p.to_vec())
}

/// The DMA grants the driver still holds.
pub fn live() -> Vec<u64> {
    dma_spans().into_iter().map(|g| g.0).collect()
}

/// Every backing the host holds lies in DMA the driver still maps.
pub fn backed_live(r: &Rig) -> bool {
    let spans = dma_spans();
    let held = |a: u64, n: u32| spans.iter().any(|&(_, b, l)| a >= b && a + n as u64 <= b + l);
    r.host.borrow().res.values().filter_map(|x| x.backing).all(|(a, n)| held(a, n))
}

/// The host turns scanout `s` to `d` (0 x 0: off) and raises the display
/// event in the device configuration's events word.
pub fn raise(r: &mut Rig, s: usize, d: (u32, u32)) {
    r.host.borrow_mut().displays[s] = d;
    r.regs.present32(EVENTS_READ, 1);
}

/// STATE for `o` as the compositor reads it.
pub fn state(r: &mut Rig, o: u16) -> nonos_gfx_wire::State {
    nonos_gfx_wire::State::decode(&ask(r, ops::STATE, o, &[]).1).expect("a state")
}
