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

//! Ctrl+V and Shift+Insert: the clipboard's first line into the field that
//! has the keyboard, by that field's own rule.

use super::paste_target::{into, target_of, Target};
use crate::settings::state::edit_paste::Pasted;
use crate::settings::state::status::StatusKind;
use crate::settings::state::State;
use nonos_app_skeleton::{clipboard_paste_line, EventOutcome};
use nonos_wifi_client::wipe;

/// More than any field here holds, so a longer line is seen to be cut.
const CLIP_MAX: usize = 256;

/// None when no field has the keyboard, so the key goes on as any other.
pub(super) fn on_paste(state: &mut State) -> Option<EventOutcome> {
    let target = target_of(state)?;
    let mut buf = [0u8; CLIP_MAX];
    let pasted = match clipboard_paste_line(&mut buf) {
        Ok(Some(line)) => Ok(into(state, target, line.text)),
        Ok(None) => Ok(Pasted::Empty),
        Err(_) => Err(()),
    };
    // The line may be a passphrase; it does not stay on the stack.
    wipe(&mut buf);
    let note: &[u8] = match (pasted, target) {
        (Err(()), _) => b"paste: the clipboard is not available",
        (Ok(Pasted::Whole), _) => return Some(EventOutcome::Repaint),
        (Ok(Pasted::Empty), _) => b"paste: the clipboard holds no text",
        (Ok(Pasted::Cut), _) => b"paste: cut to what the field holds",
        (Ok(Pasted::Refused), Target::Passphrase) => {
            b"paste refused: a passphrase is printable ASCII"
        }
        (Ok(Pasted::Refused), _) => {
            b"paste refused: letters, digits, dot, dash and underscore only"
        }
    };
    state.status.set(StatusKind::Error, note);
    Some(EventOutcome::Repaint)
}
