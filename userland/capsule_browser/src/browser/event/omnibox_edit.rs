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

use alloc::string::String;
use alloc::vec;

use nonos_app_skeleton::{clipboard_copy, clipboard_paste};

use crate::browser::omnibox::{EditKey, MAX_LEN};
use crate::browser::state::State;

/* Apply an editing key to the address bar. Returns whether the text, caret
 * or selection changed. A clipboard that is not there says so in the
 * status line instead of failing quietly. */
pub(super) fn apply(state: &mut State, k: EditKey) -> bool {
    let ed = &mut state.ui.omnibox;
    match k {
        EditKey::Insert(c) => ed.replace_selection(c.encode_utf8(&mut [0u8; 4])),
        EditKey::Backspace => ed.backspace(),
        EditKey::WordBackspace => ed.backspace_word(),
        EditKey::Delete => ed.delete(),
        EditKey::WordDelete => ed.delete_word(),
        EditKey::Left { word, extend } => ed.left(word, extend),
        EditKey::Right { word, extend } => ed.right(word, extend),
        EditKey::Home { extend } => ed.home(extend),
        EditKey::End { extend } => ed.end(extend),
        EditKey::SelectAll | EditKey::FocusOmnibox => ed.select_all(),
        EditKey::Copy | EditKey::Cut if !ed.has_selection() => return false,
        EditKey::Copy | EditKey::Cut => {
            if let Err(e) = clipboard_copy(ed.selected().as_bytes()) {
                state.status = String::from(e);
                return false;
            }
            if k == EditKey::Copy {
                return false;
            }
            ed.replace_selection("");
        }
        EditKey::Paste => {
            let mut buf = vec![0u8; MAX_LEN];
            match clipboard_paste(&mut buf) {
                Ok(n) => ed.replace_selection(&one_line(&buf[..n])),
                Err(e) => {
                    state.status = String::from(e);
                    return false;
                }
            }
        }
        EditKey::Commit | EditKey::Cancel | EditKey::Ignore => return false,
    }
    true
}

/* Pasted text on one line: line breaks go, other control characters
 * become spaces, as a browser address bar takes a multi-line paste. */
fn one_line(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    text.chars()
        .filter(|&c| c != '\r' && c != '\n')
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect()
}
