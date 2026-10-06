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

pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_SEND || state.send_stage != STAGE_FORM {
        return None;
    }
    /* The form being read for its review stays as it was read. */
    if crate::wallet::act::running(state) {
        return Some(EventOutcome::Idle);
    }
    match code {
        KEY_TAB => {
            state.send_focus =
                if state.send_focus == SEND_FIELD_TO { SEND_FIELD_AMOUNT } else { SEND_FIELD_TO };
        }
        KEY_ENTER => return Some(super::click::review(state)),
        KEY_BACKSPACE if state.send_focus == SEND_FIELD_TO => {
            state.send_to_len = state.send_to_len.saturating_sub(1);
        }
        KEY_BACKSPACE => {
            state.send_all = false;
            state.send_amount.backspace();
        }
        c if state.send_focus == SEND_FIELD_TO && (0x20..0x7F).contains(&c) => {
            if let Err(why) = push_hex(state, c) {
                state.status = why;
                return Some(EventOutcome::Repaint);
            }
        }
        c => {
            let dp = crate::wallet::send::decimals(state.send_token);
            match c {
                0x2E | 0x2C => state.send_amount.start_point(),
                0x30..=0x39 => state.send_amount.digit((c - 0x30) as u8, dp),
                _ => return None,
            }
            state.send_all = false;
        }
    }
    state.failure = None;
    Some(EventOutcome::Repaint)
}

const NOT_HEX: &[u8] = b"an address takes only 0 to 9 and a to f";
const WHOLE: &[u8] = b"the address is whole: 40 hex digits";

/// One typed character of the address. A leading "0x" is the prefix, not
/// two digits. Anything else that is not hex is refused, as is a digit
/// past the fortieth.
pub fn push_hex(state: &mut State, c: u32) -> Result<(), &'static [u8]> {
    let x = state.send_to_len == 1 && state.send_to_hex[0] == b'0';
    if x && (c == u32::from(b'x') || c == u32::from(b'X')) {
        state.send_to_len = 0;
        return Ok(());
    }
    if crate::wallet::event::hex_digit(c).is_none() {
        return Err(NOT_HEX);
    }
    if state.send_to_len >= 40 {
        return Err(WHOLE);
    }
    state.send_to_hex[state.send_to_len] = c as u8;
    state.send_to_len += 1;
    Ok(())
}

/// Ctrl+V into the address: taken only when the paste is one whole
/// address, which then replaces the field. Anything else, a hash, a cut
/// address, a sentence holding one, leaves the field as it was and says so.
pub fn paste(state: &mut State) -> Option<EventOutcome> {
    if state.view != VIEW_SEND || state.send_stage != STAGE_FORM {
        return None;
    }
    let mut buf = [0u8; 128];
    let n = nonos_app_skeleton::clients::clipboard::clipboard_paste(&mut buf).ok()?;
    let text = core::str::from_utf8(&buf[..n.min(buf.len())]).unwrap_or("");
    state.send_focus = SEND_FIELD_TO;
    match crate::wallet::event::address_text::pasted(text) {
        Ok(hex) => {
            state.send_to_hex = hex;
            state.send_to_len = 40;
            state.failure = None;
        }
        Err(why) => state.failure = Some(why),
    }
    Some(EventOutcome::Repaint)
}
