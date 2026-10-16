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

//! The controls through F 12's bytes on the two-output rig, with pipe B's
//! monitor on HDMI and DDC/CI: BRIGHTNESS lists it beside the eDP panel
//! and SET_BRIGHTNESS reaches it; VCP and SET_VCP answer for the allowed
//! controls only, sets from power or the shell alone, and the eDP panel
//! and the DP monitor without DDC/CI have none.

use super::rig::{ask, rig, HDMI};

#[test]
fn the_hdmi_monitor_is_listed_and_set_through_brightness() {
    let (mut d, model) = rig(true);
    let (status, body) = ask(&mut d, &[3], false);
    assert_eq!((status, body[0]), (0, 2), "the eDP panel and the monitor");
    assert_eq!((&body[107..111], body[111]), (&HDMI[..], 70));
    assert_eq!(ask(&mut d, &[4, 40, HDMI[0], HDMI[1], HDMI[2], HDMI[3]], true).0, 0);
    let m = model.borrow();
    assert_eq!(m.ddc.mccs.as_ref().unwrap().1.value(0x10), Some(40));
}

#[test]
fn vcp_answers_the_allowed_controls_and_sets_only_from_power_or_the_shell() {
    let (mut d, model) = rig(true);
    let at = |c: u8| [5, HDMI[0], HDMI[1], HDMI[2], HDMI[3], c];
    assert_eq!(ask(&mut d, &at(0x12), false), (0, vec![50, 0, 80, 0]));
    assert_eq!(ask(&mut d, &at(0x60), false), (0, vec![0x0F, 0, 0x12, 0]), "input, read");
    let set = |c: u8, v: u16| [6, HDMI[0], HDMI[1], HDMI[2], HDMI[3], c, v as u8, (v >> 8) as u8];
    assert_eq!(ask(&mut d, &set(0x12, 60), false).0, -1, "EPERM for anyone else");
    assert_eq!(ask(&mut d, &set(0x60, 0x11), true).0, -1, "the input source is not set");
    assert_eq!(ask(&mut d, &set(0x04, 1), true).0, -1, "a factory reset is not allowed");
    assert_eq!(ask(&mut d, &set(0x12, 81), true).0, -22, "past the maximum");
    assert_eq!(ask(&mut d, &set(0x12, 60), true).0, 0);
    assert_eq!(model.borrow().ddc.mccs.as_ref().unwrap().1.value(0x12), Some(60));
    let edp = [5, 0, 0, 0, 0x80, 0x12];
    assert_eq!(ask(&mut d, &edp, false).0, -38, "the eDP panel has no DDC/CI here");
    assert_eq!(ask(&mut d, &[5, 9, 0, 0, 0x80, 0x12], false).0, -2, "no such panel");
}

#[test]
fn a_dp_monitor_without_ddc_ci_is_not_listed() {
    let (mut d, _) = rig(false);
    let (status, body) = ask(&mut d, &[3], false);
    assert_eq!((status, body[0]), (0, 1), "the eDP panel alone");
    assert_eq!(ask(&mut d, &[5, HDMI[0], HDMI[1], HDMI[2], HDMI[3], 0x10], false).0, -38);
}
