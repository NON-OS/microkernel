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

//! An aux bring-up that stops after the keyboard port was held off must
//! turn it back on: the mouse is optional, the keyboard is not. The
//! controller here tags its configuration byte as aux data once the aux
//! clock runs, so the mouse's configuration read finds nothing and fails.

use nonos_libc::present;

use super::controller::{Keyboard, MouseKind, CONFIG_KBD_DISABLE};
use super::shared::{machine, AUX, KBD};
use crate::setup::run;

#[test]
fn a_failed_aux_bring_up_leaves_the_keyboard_port_on() {
    let tagging = Keyboard { ctr_aux_tag: true, ..Keyboard::PROMPT };
    let (ctl, _port) = machine(tagging, MouseKind::Present { wheel: false });
    present(&[KBD, AUX]);
    let driver = run().expect("ready");
    assert!(!driver.mouse_enabled, "the configuration read was expected to fail here");
    let c = ctl.borrow();
    assert_eq!(c.config & CONFIG_KBD_DISABLE, 0, "keyboard port left off: {:02x?}", c.commands());
}
