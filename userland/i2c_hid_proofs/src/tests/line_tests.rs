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

//! The interrupt line's trust lapses after five quiet seconds, and timed
//! reads come back; and a reset it rings for lets go of a mouse's buttons.

use nonos_i2cmodel::touchpad::{touch_report, Touch};
use nonos_i2cmodel::{Command, HidOverI2c};
use nonos_libc::{take_events, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP};

use super::descs::MOUSE;
use super::fixture::{of_kind, pad, PAD};
use super::wired::{wired, Wired};
use crate::drive::pass;
use crate::setup;

fn reads(w: &Wired) -> usize {
    let read = |c: &&Command| matches!(c, Command::InputRead | Command::RegisterRead(3));
    w.pad.lock().take_commands().iter().filter(read).count()
}

#[test]
fn a_line_quiet_for_five_seconds_is_no_longer_trusted() {
    let w = wired(pad());
    let mut s = setup::run().expect("setup");
    w.pad.lock().push_input(&touch_report(&Touch::finger(100, 100)));
    pass(&mut s, 1_000);
    w.pad.lock().take_commands();
    (1_001..=5_900).for_each(|t| pass(&mut s, t));
    assert_eq!(reads(&w), 0, "read with a trusted quiet line");
    (6_100..=6_300).for_each(|t| pass(&mut s, t));
    assert!(reads(&w) > 0, "a line quiet for five seconds still holds the reads back");
}

#[test]
fn a_reset_rung_for_lets_go_of_a_mouses_held_button() {
    let w = wired(HidOverI2c::new(PAD, MOUSE.to_vec(), 2 + 4, 0x04F3, 0x0002));
    let mut s = setup::run().expect("setup");
    take_events();
    w.pad.lock().push_input(&[3, 0b01, 0, 0]);
    pass(&mut s, 1_000);
    assert_eq!(of_kind(&take_events(), INPUT_KIND_BUTTON_DOWN).len(), 1);
    w.held.set(true);
    pass(&mut s, 1_020);
    w.held.set(false);
    let up = of_kind(&take_events(), INPUT_KIND_BUTTON_UP);
    assert_eq!(up.iter().map(|e| e.code).collect::<Vec<_>>(), [1], "button 1 stays held");
}
