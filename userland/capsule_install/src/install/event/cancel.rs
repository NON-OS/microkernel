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

//! Stopping a write from the keyboard.

use alloc::string::String;

use nonos_app_skeleton::EventOutcome;

use crate::install::state::{Outcome, Screen, State};

/// Stopping a write is safe at any point: the partition table goes down
/// last, so a disk abandoned here has no table and firmware reads it as
/// empty. The read-back cannot be stopped; there is nothing to save by it.
pub fn cancel(state: &mut State) -> EventOutcome {
    state.job = None;
    state.outcome = Some(Outcome {
        disk_guid: [b'-'; 36],
        partition_guid: [b'-'; 36],
        bytes_written: 0,
        bytes_verified: 0,
        seconds: 0,
        error: Some(String::from("stopped by you before the table was written")),
    });
    state.screen = Screen::Failed;
    EventOutcome::Repaint
}
