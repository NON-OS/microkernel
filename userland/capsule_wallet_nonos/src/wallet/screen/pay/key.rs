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
//! Typing on the payment form: Tab moves between the address and the
//! amount, Enter opens the review, Backspace takes back a key. The address
//! takes hex digits only, as typed, capitals kept for the checksum, and the
//! amount digits and one point; any other key is refused with a word, so
//! nothing typed can turn into a different payment than the one on the
//! screen. A paste is taken only when it is one whole address.

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_TAB};

use crate::wallet::send::STAGE_FORM;
use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_TO, VIEW_SEND};

pub use super::key_address::{paste, push_hex};

pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_SEND || state.send.stage != STAGE_FORM {
        return None;
    }
    /* The form being read for its review stays as it was read. */
    if crate::wallet::act::running(state) {
        return Some(EventOutcome::Idle);
    }
    match code {
        KEY_TAB => {
            state.send.focus =
                if state.send.focus == SEND_FIELD_TO { SEND_FIELD_AMOUNT } else { SEND_FIELD_TO };
        }
        KEY_ENTER => return Some(super::click::review(state)),
        KEY_BACKSPACE if state.send.focus == SEND_FIELD_TO => {
            state.send.to_len = state.send.to_len.saturating_sub(1);
        }
        KEY_BACKSPACE => {
            state.send.all = false;
            state.send.amount.backspace();
        }
        c if state.send.focus == SEND_FIELD_TO && (0x20..0x7F).contains(&c) => {
            if let Err(why) = push_hex(state, c) {
                state.status = why;
                return Some(EventOutcome::Repaint);
            }
        }
        c => {
            let dp = crate::wallet::send::decimals(state.send.token);
            match c {
                0x2E | 0x2C => state.send.amount.start_point(),
                0x30..=0x39 => state.send.amount.digit((c - 0x30) as u8, dp),
                _ => return None,
            }
            state.send.all = false;
        }
    }
    state.failure = None;
    Some(EventOutcome::Repaint)
}
