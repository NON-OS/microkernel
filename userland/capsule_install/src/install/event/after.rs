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

//! After the install: restart, close, or go back and pick another disk.
//! Restart is the kernel's reboot, which needs the Admin bit this capsule
//! carries for exactly this one call.

use nonos_app_skeleton::{EventOutcome, KEY_ENTER, KEY_ESC};
use nonos_libc::mk_admin_reboot;

use super::router::back;
use crate::install::state::{Screen, State};

pub fn on_after_key(state: &mut State, code: u32) -> EventOutcome {
    match state.screen {
        Screen::Done => match code {
            KEY_ESC => EventOutcome::Close,
            KEY_ENTER => {
                mk_admin_reboot();
                EventOutcome::Idle
            }
            _ => EventOutcome::Idle,
        },
        _ => match code {
            KEY_ESC => EventOutcome::Close,
            KEY_ENTER => back(state, Screen::Disks),
            _ => EventOutcome::Idle,
        },
    }
}
