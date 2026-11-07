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

//! FLUSH on an engine holding an interrupt: the flip sleeps once on pipe
//! A's vertical blank, and a machine whose interrupt never comes costs
//! one park, then the rest of its flushes are polled.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::Rect;

use crate::display::Intel;
use crate::engine::flip_park::FRAME_MS;
use crate::engine::park::Blank;
use crate::engine::pipe::Pipe;
use crate::tests::park_rig::Blanks;
use crate::tests::wire_rig::{rig, shadow_bytes};

type Parked = Intel<crate::model::Shared, crate::model::ModelFrames, crate::model::Ticks, Blanks>;

fn parked(delivers: bool) -> (Parked, std::rc::Rc<std::cell::RefCell<crate::model::Gen9>>) {
    let (d, model) = rig();
    let mut park = Blanks::new(model.clone());
    park.delivers = delivers;
    model.borrow_mut().period = 0;
    let Intel { m, clock, gen, heads, .. } = d;
    (Intel::new(m, clock, gen, heads, park), model)
}

fn flush(d: &mut Parked, px: &[u8], x: u32) {
    let s = Shadow { bytes: px, width: 1920, height: 1080, stride: 7680, cursor_offset: 0 };
    d.flush(0, &[Rect { x, y: 0, width: 8, height: 8 }], &s).unwrap();
}

#[test]
fn a_flush_sleeps_on_the_pipe_blank() {
    let (mut d, model) = parked(true);
    flush(&mut d, &shadow_bytes(0xFF00_0011), 0);
    flush(&mut d, &shadow_bytes(0xFF00_0022), 100);
    assert_eq!((d.park.waits.as_slice(), d.park.acks), (&[FRAME_MS, FRAME_MS][..], 2));
    assert_eq!(d.blank[Pipe::A.index() as usize], Blank::Irq);
    let m = model.borrow();
    assert_eq!(m.pixel(Pipe::A, 3, 3), Some(0xFF00_0011));
    assert_eq!(m.pixel(Pipe::A, 103, 3), Some(0xFF00_0022));
}

#[test]
fn a_silent_interrupt_costs_one_park_then_polls() {
    let (mut d, model) = parked(false);
    flush(&mut d, &shadow_bytes(0xFF00_0011), 0);
    assert_eq!(d.blank[Pipe::A.index() as usize], Blank::Poll);
    model.borrow_mut().period = 3;
    flush(&mut d, &shadow_bytes(0xFF00_0022), 100);
    assert_eq!((d.park.waits.len(), d.park.acks), (1, 0));
    assert_eq!(model.borrow().pixel(Pipe::A, 103, 3), Some(0xFF00_0022));
}
