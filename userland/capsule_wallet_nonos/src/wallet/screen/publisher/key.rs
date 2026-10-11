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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER};

use crate::wallet::state::{State, VIEW_PUBLISHER};

/// The longest path typed; the screen refuses one past 200 on Open.
const PATH_MAX: usize = 255;

/// A key on the publish screen: printable characters into the path,
/// Backspace, and Enter to open the document.
pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_PUBLISHER {
        return None;
    }
    let path = &mut state.publisher.path;
    match code {
        KEY_ENTER => return Some(super::click::read(state)),
        KEY_BACKSPACE => {
            path.pop();
        }
        c if (0x21..0x7F).contains(&c) && path.len() < PATH_MAX => path.push(c as u8 as char),
        _ => return None,
    }
    state.failure = None;
    Some(EventOutcome::Repaint)
}
