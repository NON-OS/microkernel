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

//! A pad behind the wired controller that power-cycles while a click is
//! held, refusing its first writes, and the quiet passes after.

use nonos_i2cmodel::touchpad::{touch_report, Touch, MODE_REPORT_ID};
use nonos_i2cmodel::HidOverI2c;
use nonos_libc::{mk_idle_ms, take_events, INPUT_KIND_BUTTON_UP};

use crate::drive::pass;
use crate::setup;
use crate::state::State;
use crate::tests::fixture::of_kind;
use crate::tests::wired::{wired, Wired};

pub(super) const PRESS: Touch =
    Touch { x: 0, y: 0, tip: true, confidence: true, contacts: 1, button: true, id: 0 };

/// `device` bound, a click held, then a power cycle: ringing, its mode the
/// mouse's, its first `refusals` writes refused.
pub(super) fn rung_and_refused(device: HidOverI2c, refusals: u32) -> (Wired, State) {
    let w = wired(device);
    let mut s = setup::run().expect("setup");
    for (t, x) in [(1_000, 300), (1_010, 301)] {
        w.pad.lock().push_input(&touch_report(&Touch { x, ..PRESS }));
        pass(&mut s, t);
    }
    take_events();
    w.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID, 0, 0]);
    w.nacks.set(refusals);
    w.held.set(true);
    pass(&mut s, 1_020);
    w.held.set(false);
    let ups = of_kind(&take_events(), INPUT_KIND_BUTTON_UP).len();
    assert!(ups == 1 && w.nacks.get() < refusals, "{ups} button-ups, or no write refused");
    (w, s)
}

/// Passes 10 ms apart with nothing to read, from `from` up to `to` ms.
pub(super) fn quiet(s: &mut State, from: u64, to: u64) {
    (from / 10..to / 10).for_each(|t| {
        mk_idle_ms(10);
        pass(s, t * 10);
    });
}
