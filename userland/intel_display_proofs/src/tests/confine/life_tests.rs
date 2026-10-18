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
//! A driver's life on armed rigs: flips, cursor shapes and moves, mode
//! sets there and back, and a monitor pulled mid-set and plugged back. The
//! table stays confined throughout.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::body::cursor::{NEW_SHAPE, VISIBLE};
use nonos_gfx_wire::{Cursor, Rect};

use super::kit::confined;
use crate::engine::aux_regs::Ddi;
use crate::engine::dpll::DPLL_CTRL1;
use crate::tests::deaths::kit::rig;
use crate::tests::plug::plug_rig::{settle, watched, with_shadow};
use crate::tests::wire_rig::Rig;

fn flush(d: &mut Rig, w: u32, h: u32) {
    with_shadow(w, h, |s: &Shadow| d.flush(0, &[Rect { x: 0, y: 0, width: w, height: h }], s))
        .unwrap();
}

fn cursor(d: &mut Rig, flags: u32, x: i32) {
    let c = Cursor { flags, x, y: 40, hot_x: 0, hot_y: 0, width: 64, height: 64 };
    d.cursor(0, &c, &vec![0x7Fu8; 64 * 64 * 4]).unwrap();
}

#[test]
fn flips_cursors_and_mode_sets_there_and_back_stay_confined() {
    let (mut d, model) = rig();
    for _ in 0..3 {
        flush(&mut d, 1920, 1080);
    }
    cursor(&mut d, NEW_SHAPE | VISIBLE, 100);
    cursor(&mut d, VISIBLE, 300);
    with_shadow(1024, 768, |s| d.set_mode(0, 1024, 768, s)).unwrap();
    flush(&mut d, 1024, 768);
    with_shadow(1920, 1080, |s| d.set_mode(0, 1920, 1080, s)).unwrap();
    flush(&mut d, 1920, 1080);
    confined(&model.borrow(), "a life on one head");
}

#[test]
fn a_monitor_pulled_mid_set_and_plugged_back_stays_confined() {
    let (mut d, model, sink) = watched();
    model.borrow_mut().cable.pull_on = Some(DPLL_CTRL1);
    assert!(with_shadow(1024, 768, |s| d.set_mode(0, 1024, 768, s)).is_err());
    confined(&model.borrow(), "pulled mid-set");
    settle(&mut d);
    model.borrow_mut().plug_in(Ddi::B, sink);
    settle(&mut d);
    with_shadow(1920, 1080, |s| d.set_mode(0, 1920, 1080, s)).unwrap();
    confined(&model.borrow(), "plugged back and set");
}
