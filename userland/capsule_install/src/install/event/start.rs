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

//! The two screens before a disk is chosen. Enter on the first shows what
//! this boot proved, with the capsules counted again so the count is of now;
//! Enter there lists the disks, when there is an image to write.

use nonos_app_skeleton::{EventOutcome, KEY_ENTER, KEY_ESC};

use crate::install::source::census;
use crate::install::state::{Screen, State};
use crate::install::survey::look;

pub fn on_start_key(state: &mut State, code: u32) -> EventOutcome {
    match (state.screen, code) {
        (Screen::Welcome, KEY_ESC) => EventOutcome::Close,
        (Screen::Welcome, KEY_ENTER) => {
            state.boot.capsules = census();
            state.screen = Screen::Proofs;
            EventOutcome::Repaint
        }
        (Screen::Proofs, KEY_ESC) => {
            state.screen = Screen::Welcome;
            EventOutcome::Repaint
        }
        (Screen::Proofs, KEY_ENTER) if state.image.is_some() => {
            look(state);
            state.screen = Screen::Disks;
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}
