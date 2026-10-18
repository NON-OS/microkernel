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

//! The lock LEDs the serve loop sends at start, before any lock key: the
//! keypad types digits, so Num Lock shows lit from the first sync.

use super::controller::{Keyboard, MouseKind};
use super::serving::serving;
use crate::constants::KBD_SET_LEDS;
use crate::keymap::locks::LED_NUM;
use crate::server::upkeep::sync_leds;

#[test]
fn the_first_sync_lights_num_lock_and_nothing_else() {
    let (ctl, _port, mut ctx) = serving(Keyboard::PROMPT, MouseKind::Absent);
    sync_leds(&mut ctx);
    assert_eq!(ctl.borrow().data_writes(), [KBD_SET_LEDS, LED_NUM]);
    sync_leds(&mut ctx);
    assert_eq!(ctl.borrow().data_writes().len(), 2, "the same LEDs were sent twice");
}
