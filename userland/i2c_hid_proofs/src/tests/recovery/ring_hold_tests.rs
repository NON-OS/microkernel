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

//! The zero-length report a reset rings for, inside the holdoff a stick
//! report started: the click is let go at the ring, and the set-up the
//! reset is owed runs once the holdoff has passed, with no report to ask,
//! and only once.

use nonos_i2cmodel::touchpad::{touch_report, Touch, MODE_REPORT_ID};
use nonos_i2cmodel::{Command, HidOverI2c};
use nonos_libc::{mk_idle_ms, take_events, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP};

use crate::drive::pass;
use crate::setup;
use crate::tests::descs::pad_with_mouse;
use crate::tests::fixture::{of_kind, MAX_INPUT, PAD};
use crate::tests::wired::wired;

#[test]
fn a_reset_rung_for_inside_the_holdoff_lets_go_at_the_ring() {
    let w = wired(HidOverI2c::new(PAD, pad_with_mouse(), MAX_INPUT, 0x04F3, 0x3028));
    let mut s = setup::run().expect("setup");
    take_events();
    let press = Touch { button: true, ..Touch::finger(300, 300) };
    w.pad.lock().push_input(&touch_report(&press));
    pass(&mut s, 1_000);
    w.pad.lock().push_input(&touch_report(&Touch { x: 301, ..press }));
    pass(&mut s, 1_010);
    assert_eq!(of_kind(&take_events(), INPUT_KIND_BUTTON_DOWN).len(), 1, "no click held");
    // A stick report: the mode is read back, and the holdoff starts.
    w.pad.lock().push_input(&[3, 0, 1, 0]);
    pass(&mut s, 1_020);
    take_events();
    // The pad resets and rings for its zero-length report.
    w.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID, 0, 0]);
    w.held.set(true);
    pass(&mut s, 1_040);
    w.held.set(false);
    let ups = of_kind(&take_events(), INPUT_KIND_BUTTON_UP).len();
    assert_eq!(ups, 1, "the click was not let go at the ring");
    let mode = || w.pad.lock().feature(MODE_REPORT_ID).map(<[u8]>::to_vec);
    assert_eq!(mode(), Some(vec![MODE_REPORT_ID, 0, 0]), "set up inside the holdoff");
    // Past the holdoff, a pass with nothing to read sets the pad up.
    mk_idle_ms(1_100);
    pass(&mut s, 2_200);
    assert_eq!(mode(), Some(vec![MODE_REPORT_ID, 3, 0b11]), "the owed set-up was dropped");
    // It is owed once: later holdoffs pass with nothing asked of the pad.
    w.pad.lock().take_commands();
    (1..=3).for_each(|i| {
        mk_idle_ms(1_100);
        pass(&mut s, 2_200 + i * 1_100);
    });
    let asked = w.pad.lock().commands().iter().any(|c| matches!(c, Command::GetReport { .. }));
    assert!(!asked, "set up again with nothing owed");
}
