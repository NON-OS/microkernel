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

//! The pad and its mouse collection bound behind the faulty controller,
//! a click held down and dragged, and fed mouse reports.

use nonos_i2cmodel::touchpad::{touch_report, Touch};
use nonos_i2cmodel::HidOverI2c;
use nonos_libc::{mk_idle_ms, take_events, INPUT_KIND_BUTTON_DOWN};

use super::faulty::{faulty, Faulty};
use crate::input::poll;
use crate::setup;
use crate::state::State;
use crate::tests::descs::pad_with_mouse;
use crate::tests::fixture::{of_kind, MAX_INPUT, PAD};

pub fn dragging() -> (Faulty, State) {
    let f = faulty(HidOverI2c::new(PAD, pad_with_mouse(), MAX_INPUT, 0x04F3, 0x3028));
    let mut s = setup::run().expect("setup");
    let press = Touch { button: true, ..Touch::finger(300, 300) };
    for t in [press, Touch { x: 301, ..press }] {
        f.pad.lock().push_input(&touch_report(&t));
        poll(&mut s);
    }
    assert_eq!(of_kind(&take_events(), INPUT_KIND_BUTTON_DOWN).len(), 1, "no click held");
    (f, s)
}

/// `n` mouse reports, 8 ms apart.
pub fn mouse_reports(f: &Faulty, s: &mut State, n: u8) {
    for dx in 1..=n {
        f.pad.lock().push_input(&[3, 0, dx, 0]);
        poll(s);
        mk_idle_ms(8);
    }
}
