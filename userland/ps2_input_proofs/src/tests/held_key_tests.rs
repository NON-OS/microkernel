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

//! A key held down while the mouse comes up, as on a laptop booted with a
//! finger still on a key: its bytes share the output buffer with the
//! mouse's replies and are told apart only by the AUXDATA status bit.

use nonos_libc::present;

use super::controller::{Keyboard, MouseKind, CONFIG_KBD_DISABLE};
use super::shared::{machine, AUX, KBD};
use crate::constants::CONFIG_AUX_DISABLE;
use crate::setup::run;

/// Set 1 make code of the A key.
const KEY_A: u8 = 0x1E;

#[test]
fn a_key_held_through_the_mouse_bring_up_does_not_cost_the_touchpad() {
    let held = Keyboard { held: Some(KEY_A), ..Keyboard::PROMPT };
    let (ctl, _port) = machine(held, MouseKind::Present { wheel: true });
    present(&[KBD, AUX]);
    let driver = run().expect("ready");
    assert!(driver.mouse_enabled, "a keystroke was taken for the mouse's ACK");
    assert!(driver.mouse_wheel, "a keystroke was taken for the mouse's id");
    let c = ctl.borrow();
    assert!(c.mouse.reporting);
    assert!(crate::init::dropped() > 0, "the held key's bytes were not counted as dropped");
    assert_eq!(c.config & (CONFIG_KBD_DISABLE | CONFIG_AUX_DISABLE), 0, "both ports clocked");
}
