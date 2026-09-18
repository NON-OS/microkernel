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

//! Which key does what, on which screen. Escape always steps back, and
//! closes the window from the first screen or the last; while a disk is
//! being written no key does anything, because there is nothing safe a key
//! could do to a half-written disk except wait.

use super::after::on_after_key;
use super::cancel::cancel;
use super::confirm::on_confirm_key;
use crate::install::state::{Screen, State};
use nonos_app_skeleton::{
    EventOutcome, InputEvent, InputKind, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_UP,
};
use nonos_blk_client::scan;

pub fn on_event(state: &mut State, e: InputEvent) -> EventOutcome {
    if e.kind != InputKind::KeyDown {
        return EventOutcome::Idle;
    }
    match state.screen {
        Screen::Welcome => match e.code {
            KEY_ESC => EventOutcome::Close,
            KEY_ENTER if state.image.is_some() => {
                state.disks = scan();
                state.selected = 0;
                state.screen = Screen::Disks;
                EventOutcome::Repaint
            }
            _ => EventOutcome::Idle,
        },
        Screen::Disks => match e.code {
            KEY_ESC => back(state, Screen::Welcome),
            KEY_UP if state.selected > 0 => {
                state.selected -= 1;
                EventOutcome::Repaint
            }
            KEY_DOWN if state.selected + 1 < state.disks.len() => {
                state.selected += 1;
                EventOutcome::Repaint
            }
            KEY_ENTER if state.selected_disk().is_some_and(|d| d.device.is_some()) => {
                state.typed.clear();
                state.screen = Screen::Confirm;
                EventOutcome::Repaint
            }
            _ => EventOutcome::Idle,
        },
        Screen::Confirm => on_confirm_key(state, e.code),
        Screen::Writing if e.code == KEY_ESC => cancel(state),
        Screen::Writing | Screen::Verifying => EventOutcome::Idle,
        Screen::Done | Screen::Failed => on_after_key(state, e.code),
    }
}

pub(super) fn back(state: &mut State, to: Screen) -> EventOutcome {
    state.screen = to;
    EventOutcome::Repaint
}
