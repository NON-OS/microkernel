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

/// The status setup exits with when the person chose to install NONOS.
const EXIT_INSTALLER: i32 = 3;
/* The status of a setup the person finished with another mode. */
const EXIT_DESKTOP: i32 = 0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Ended {
    /// Start the desktop.
    Desktop,
    /// Start the desktop and open the installer.
    Installer,
    /* Setup never started, failed, or its status is gone: nobody chose. */
    Unfinished,
}

/// `None` while setup runs.
pub fn ended() -> Option<Ended> {
    if super::shared_state().is_alive() {
        return None;
    }
    let pid = super::state::spawned_pid();
    let code = match get_process(pid) {
        Some(pcb) => match *pcb.state.lock() {
            ProcessState::Zombie(code) | ProcessState::Terminated(code) => Some(code),
            _ => None,
        },
        None => crate::process::exit::peek_exit_status(pid),
    };
    Some(match (pid, code) {
        (0, _) => Ended::Unfinished,
        (_, Some(EXIT_INSTALLER)) => Ended::Installer,
        (_, Some(EXIT_DESKTOP)) => Ended::Desktop,
        _ => Ended::Unfinished,
    })
}
