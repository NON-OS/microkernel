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

//! A paste into a running program: bracketed and stripped of escapes for a
//! raw reader, typed line by line for a line reader.

use alloc::string::String;
use alloc::vec;

use nonos_app_skeleton::{clipboard_paste, EventOutcome};
use nonos_vt::input::encode_paste;

use super::cooked::Effect;
use super::fg_input::{reads_raw, send};
use crate::term::state::State;

/// Largest paste taken at once.
const PASTE_MAX: usize = 16 * 1024;

pub(super) fn paste_to_program(state: &mut State) -> EventOutcome {
    let mut buf = vec![0u8; PASTE_MAX];
    let n = match clipboard_paste(&mut buf) {
        Ok(n) => n.min(buf.len()),
        Err(_) => {
            state.scrollback.push_line(b"paste: clipboard unavailable");
            return EventOutcome::Repaint;
        }
    };
    let text = String::from_utf8_lossy(&buf[..n]).into_owned();
    if reads_raw(&state.scrollback.vt) {
        let mut bytes = alloc::vec::Vec::new();
        encode_paste(&text, &state.scrollback.vt.modes, &mut bytes);
        send(state, &bytes);
        return EventOutcome::Repaint;
    }
    // A line reader gets the paste as if typed: each line sent on its end.
    let mut fx = Effect::default();
    for c in text.chars() {
        match c {
            '\r' | '\n' => state.cooked.enter(&mut fx),
            c if c.is_control() && c != '\t' => {}
            c => state.cooked.char(c, &mut fx),
        }
    }
    state.scrollback.feed_raw(&fx.echo);
    send(state, &fx.send);
    EventOutcome::Repaint
}
