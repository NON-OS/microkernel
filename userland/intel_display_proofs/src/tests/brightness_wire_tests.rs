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

//! F 12's brightness bytes served natively, on the two-output model (eDP
//! on pipe A, DP on pipe B): one panel listed under its own id with the top
//! bit set, levels 1..=100, a set to the nearest level, the sender rule,
//! and each refusal F's framing names.

use super::two_rig::two_rig;
use crate::display::{brightness_answer, Rights};
use crate::engine::backlight::BLC_PWM_PCH_CTL2;

fn ask(d: &mut super::wire_rig::Rig, req: &[u8], may: bool) -> (i32, Vec<u8>) {
    let mut out = [0u8; 512];
    let n = brightness_answer(d, 2, req, Rights { set: may, read_controls: may }, &mut out);
    (i32::from_le_bytes(out[..4].try_into().unwrap()), out[4..n].to_vec())
}

#[test]
fn the_edp_panel_is_listed_with_its_level_and_a_hundred_steps() {
    let (mut d, model, _) = two_rig();
    model.borrow_mut().set(BLC_PWM_PCH_CTL2, (4800 << 16) | 1200);
    let (status, body) = ask(&mut d, &[3], false);
    assert_eq!((status, body.len()), (0, 1 + 4 + 1 + 1 + 100));
    assert_eq!(body[0], 1, "only the eDP panel is backlit");
    assert_eq!(&body[1..5], &0x8000_0000u32.to_le_bytes(), "output 0, top bit set");
    assert_eq!((body[5], body[6]), (25, 100), "1200 of 4800 is 25 percent");
    assert_eq!((body[7], body[106]), (1, 100));
}

#[test]
fn a_set_goes_to_the_nearest_level_and_only_from_power_or_the_shell() {
    let (mut d, model, _) = two_rig();
    model.borrow_mut().set(BLC_PWM_PCH_CTL2, 4800 << 16);
    assert_eq!(ask(&mut d, &[4, 60], false), (-1, vec![]), "EPERM for anyone else");
    assert_eq!(model.borrow().get(BLC_PWM_PCH_CTL2) & 0xFFFF, 0);
    assert_eq!(ask(&mut d, &[4, 60], true), (0, vec![]));
    assert_eq!(model.borrow().get(BLC_PWM_PCH_CTL2) & 0xFFFF, 2880);
    let panel = 0x8000_0000u32.to_le_bytes();
    assert_eq!(ask(&mut d, &[4, 0, panel[0], panel[1], panel[2], panel[3]], true).0, 0);
    assert_eq!(model.borrow().get(BLC_PWM_PCH_CTL2) & 0xFFFF, 48, "0 goes to 1 percent");
}

#[test]
fn every_refusal_is_the_one_f_names() {
    let (mut d, model, _) = two_rig();
    assert_eq!(ask(&mut d, &[3], true), (-2, vec![]), "ENOENT: no PWM set up");
    assert_eq!(ask(&mut d, &[4, 50], true).0, -2);
    model.borrow_mut().set(BLC_PWM_PCH_CTL2, 4800 << 16);
    assert_eq!(ask(&mut d, &[4, 101], true).0, -22, "past 100");
    assert_eq!(ask(&mut d, &[4], true).0, -22, "no percent");
    assert_eq!(ask(&mut d, &[4, 50, 1, 0, 0, 0], true).0, -2, "a panel not listed");
    assert_eq!(ask(&mut d, &[4, 50, 1], true).0, -22, "a cut panel id");
    assert_eq!(ask(&mut d, &[9], true), (-22, vec![]), "unknown op, no body");
}
