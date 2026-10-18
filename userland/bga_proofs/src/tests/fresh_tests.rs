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
//! A driver restarted onto the adapter its killed self left (G-72, G-78),
//! through the bring-up's `fresh`: video memory zero to its last byte, and
//! already zero when the fence runs (a fence here that looks proves it),
//! before any mode is set; the mode left on kept, one cut off redone.

use std::cell::RefCell;

use crate::constants::{DISPI_INDEX_ENABLE, DISPI_INDEX_XRES, DISPI_INDEX_YRES};
use crate::dispi::{current, dispi_off, set_mode, DispiMode};
use crate::model::Adapter;
use crate::regs::Regs;
use crate::setup::fresh::fresh;

thread_local! {
    static AT: RefCell<(u64, u64, usize)> = const { RefCell::new((0, 0, 0)) };
    static SAW: RefCell<Vec<(bool, bool)>> = const { RefCell::new(Vec::new()) };
}

/// The fence: whether a mode was on, and video memory zero, as it ran.
fn look() {
    let (bar, vram, len) = AT.with(|a| *a.borrow());
    let on = current(Regs::new(bar)).is_some();
    // SAFETY: the test's video memory, alive for the call.
    let mem = unsafe { std::slice::from_raw_parts(vram as *const u8, len) };
    let zero = mem.iter().all(|&b| b == 0);
    SAW.with(|s| s.borrow_mut().push((on, zero)));
}

fn restart(a: &mut Adapter) -> Option<DispiMode> {
    a.vram.iter_mut().enumerate().for_each(|(i, b)| *b = i as u8 | 1);
    let at = (a.bar.base(), a.vram.as_mut_ptr() as u64, a.vram.len());
    AT.with(|x| *x.borrow_mut() = at);
    SAW.with(|s| s.borrow_mut().clear());
    let m = fresh(a.regs(), (at.1, at.2 as u64), &[], look).ok()?.mode;
    assert!(a.vram.iter().all(|&b| b == 0), "every byte, past the mode's too");
    Some(m)
}

#[test]
fn its_last_frame_is_gone_and_its_mode_kept() {
    let mut a = Adapter::new();
    set_mode(a.regs(), 800, 600);
    let m = restart(&mut a);
    assert_eq!(m, Some(DispiMode { width: 800, height: 600, stride: 3200 }));
    assert_eq!(SAW.with(|s| s.borrow().clone()), [(true, true)], "zero when made visible");
}

#[test]
fn a_mode_set_cut_off_part_way_starts_over_after_the_wipe() {
    let mut a = Adapter::new();
    set_mode(a.regs(), 800, 600);
    let half = [(DISPI_INDEX_ENABLE, 0), (DISPI_INDEX_XRES, 1920), (DISPI_INDEX_YRES, 1200)];
    // SAFETY: indices in the DISPI block the model maps.
    half.iter().for_each(|&(i, v)| unsafe { a.regs().w16(dispi_off(i), v) });
    let m = restart(&mut a);
    assert_eq!(m, Some(DispiMode { width: 1024, height: 768, stride: 4096 }));
    assert_eq!(current(a.regs()), m, "set whole");
    assert_eq!(SAW.with(|s| s.borrow().clone()), [(false, true)], "no mode on until wiped");
}
