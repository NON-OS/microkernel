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
//! The address field: one typed character at a time, or one whole pasted
//! address.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::send::STAGE_FORM;
use crate::wallet::state::{State, SEND_FIELD_TO, VIEW_SEND};

const NOT_HEX: &[u8] = b"an address takes only 0 to 9 and a to f";
const WHOLE: &[u8] = b"the address is whole: 40 hex digits";

/// One typed character of the address. A leading "0x" is the prefix, not
/// two digits. Anything else that is not hex is refused, as is a digit
/// past the fortieth.
pub fn push_hex(state: &mut State, c: u32) -> Result<(), &'static [u8]> {
    let x = state.send.to_len == 1 && state.send.to_hex[0] == b'0';
    if x && (c == u32::from(b'x') || c == u32::from(b'X')) {
        state.send.to_len = 0;
        return Ok(());
    }
    if crate::wallet::event::hex_digit(c).is_none() {
        return Err(NOT_HEX);
    }
    if state.send.to_len >= 40 {
        return Err(WHOLE);
    }
    state.send.to_hex[state.send.to_len] = c as u8;
    state.send.to_len += 1;
    Ok(())
}

/// Ctrl+V into the address: taken only when the paste is one whole
/// address, which then replaces the field. Anything else, a hash, a cut
/// address, a sentence holding one, leaves the field as it was and says so.
pub fn paste(state: &mut State) -> Option<EventOutcome> {
    if state.view != VIEW_SEND || state.send.stage != STAGE_FORM {
        return None;
    }
    let mut buf = [0u8; 128];
    let n = nonos_app_skeleton::clients::clipboard::clipboard_paste(&mut buf).ok()?;
    let text = core::str::from_utf8(&buf[..n.min(buf.len())]).unwrap_or("");
    state.send.focus = SEND_FIELD_TO;
    match crate::wallet::event::address_text::pasted(text) {
        Ok(hex) => {
            state.send.to_hex = hex;
            state.send.to_len = 40;
            state.failure = None;
        }
        Err(why) => state.failure = Some(why),
    }
    Some(EventOutcome::Repaint)
}
