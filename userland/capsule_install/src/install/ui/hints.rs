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

/*
 * The keys that do something on this screen, and nothing else: a person
 * should never have to guess what Enter does on an installer. Full screen,
 * leaving starts the desktop, and after the install only a restart is
 * offered, since nothing else runs until the machine boots its new disk.
 */

use crate::install::event::stoppable;
use crate::install::full::active;
use crate::install::state::{Screen, State};

pub fn hints(state: &State) -> (&'static str, &'static str) {
    let leave = if active() { "Esc leave, the desktop starts" } else { "Esc close" };
    match state.screen {
        Screen::Welcome => (leave, "Enter see the proofs"),
        Screen::Proofs if state.image.is_some() => ("Esc back", "Enter choose a disk"),
        Screen::Proofs => ("Esc back", ""),
        Screen::Disks if state.disks.iter().any(|d| d.device.is_some()) => {
            ("Esc back", "R look again   ↑↓ select   Enter continue")
        }
        Screen::Disks => ("Esc back", "R look again"),
        Screen::Confirm if matches!(state.prepared, Some(Ok(_))) => {
            ("Esc back", "type the word, then Enter")
        }
        Screen::Confirm => ("Esc back", ""),
        Screen::Writing if stoppable(state) => {
            ("Esc stop, the disk is left without a table", "do not power off")
        }
        Screen::Writing => ("", "writing the table, do not power off"),
        Screen::Verifying => ("", "do not power off"),
        Screen::Done if active() => ("", "Enter restart now"),
        Screen::Done => ("Esc close", "Enter restart now"),
        Screen::Failed => (leave, "Enter choose another disk"),
    }
}
