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

//! The fault path on DISPI (G 12): an adapter that stopped showing bga's
//! mode, turned off or set to another size behind its back, is set again
//! before STATE answers, which says lost with a new generation so the
//! compositor repaints; the next STATE is quiet again.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};

use crate::constants::DISPI_INDEX_ENABLE;
use crate::dispi::{current, dispi_off, set_mode, DispiMode};
use crate::edid::Panel;
use crate::model::Adapter;

const XGA: DispiMode = DispiMode { width: 1024, height: 768, stride: 4096 };

#[test]
fn dispi_turned_off_is_set_again_and_reported_lost_once() {
    let mut a = Adapter::new();
    set_mode(a.regs(), 1024, 768);
    let mut b = a.bga(Panel::default(), XGA);
    // SAFETY: the ENABLE index lies in the model's DISPI block.
    unsafe { a.regs().w16(dispi_off(DISPI_INDEX_ENABLE), 0) };
    assert_eq!(current(a.regs()), None);
    let s = b.state(0).unwrap();
    assert_eq!((s.flags, s.generation), (SCANNING_OUT | LOST, 2));
    assert_eq!(current(a.regs()), Some(XGA), "the mode is back before the answer");
    let s = b.state(0).unwrap();
    assert_eq!((s.flags, s.generation), (SCANNING_OUT, 2));
}

#[test]
fn another_size_set_behind_its_back_is_put_back() {
    let mut a = Adapter::new();
    set_mode(a.regs(), 1024, 768);
    let mut b = a.bga(Panel::default(), XGA);
    set_mode(a.regs(), 800, 600);
    let s = b.state(0).unwrap();
    assert_eq!((s.width, s.height, s.flags), (1024, 768, SCANNING_OUT | LOST));
    assert_eq!(current(a.regs()).map(|m| (m.width, m.height)), Some((1024, 768)));
}
