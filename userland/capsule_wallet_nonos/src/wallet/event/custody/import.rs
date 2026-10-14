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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use zeroize::Zeroize;

use super::import_submit::submit;
use crate::wallet::state::{State, VIEW_RECEIVE};

// Open or close the private-key import field. Closing always wipes whatever was
// typed so a cancelled import leaves no key material behind.
pub fn toggle_import(state: &mut State) -> EventOutcome {
    state.import_active = !state.import_active;
    if state.import_active {
        state.view = VIEW_RECEIVE;
        state.status = b"paste a 0x private key, Enter to import";
    } else {
        wipe(state);
    }
    EventOutcome::Repaint
}

// Feed one key into the import field. Hex digits accumulate up to 64 nibbles;
// Enter submits, Esc cancels, Backspace deletes. The typed value is never
// rendered, only counted.
pub fn import_input(state: &mut State, code: u32) -> EventOutcome {
    if code == KEY_ESC {
        wipe(state);
        state.import_active = false;
        state.status = b"import cancelled";
        return EventOutcome::Repaint;
    }
    if code == KEY_ENTER {
        return submit(state);
    }
    if code == KEY_BACKSPACE {
        if state.custody.import_len > 0 {
            state.custody.import_len -= 1;
            state.custody.import_hex[state.custody.import_len] = 0;
        }
        return EventOutcome::Repaint;
    }
    // Accept a leading 0x without consuming a nibble slot.
    if state.custody.import_len == 0 && (code == b'x' as u32 || code == b'X' as u32) {
        return EventOutcome::Repaint;
    }
    if super::super::hex_digit::hex_digit(code).is_some()
        && state.custody.import_len < state.custody.import_hex.len()
    {
        state.custody.import_hex[state.custody.import_len] = code as u8;
        state.custody.import_len += 1;
    }
    EventOutcome::Repaint
}

pub(in crate::wallet::event) fn wipe(state: &mut State) {
    state.custody.import_hex.zeroize();
    state.custody.import_len = 0;
}
