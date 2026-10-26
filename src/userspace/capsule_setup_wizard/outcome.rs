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

//! What setup asked for when it ended.
//!
//! Setup cannot start an app: the desktop that would show one starts only once
//! setup has exited. So its exit status carries the request, and init acts on
//! it after starting the desktop.

use crate::process::{get_process, ProcessState};

use super::outcome_rule::read;
pub use super::outcome_rule::Ended;

/*
 * `None` while setup runs. The status's low byte says what setup asked for,
 * and the byte above it the apps the person turned off:
 * none for a setup that did not finish, so every app starts.
 */
pub fn ended() -> Option<(Ended, u8)> {
    if super::shared_state().is_alive() {
        return None;
    }
    let pid = super::state::spawned_pid();
    let pcb = get_process(pid).and_then(|pcb| match *pcb.state.lock() {
        ProcessState::Zombie(code) | ProcessState::Terminated(code) => Some(code),
        _ => None,
    });
    let logged = crate::process::exit::peek_exit_status(pid);
    Some(read(pid, crate::process::exit::kept_status(pid), pcb, logged))
}
