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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER};

use crate::wallet::nox::NOX_DECIMALS;
use crate::wallet::publish::{PublisherUi, FOCUS_AGENT, FOCUS_AMOUNT, FOCUS_PATH, FOCUS_URI};
use crate::wallet::state::{State, VIEW_PUBLISHER};

/// The longest path typed; the screen refuses one past 200 on Open.
const PATH_MAX: usize = 255;
/// An address with its 0x; a uri one past MAX_URL, so a longer one is
/// refused on its press rather than cut.
const AGENT_MAX: usize = 42;
const URI_MAX: usize = 257;

/// The focused text field and its longest.
fn text(p: &mut PublisherUi) -> (&mut String, usize) {
    match p.focus {
        FOCUS_AGENT => (&mut p.agent, AGENT_MAX),
        FOCUS_URI => (&mut p.uri, URI_MAX),
        _ => (&mut p.path, PATH_MAX),
    }
}

/// A key on the publish screen, into the focused field: printable
/// characters, Backspace, and Enter to open the document from the path.
/// The bond amount takes digits and one point, in NOX. A review takes none.
pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_PUBLISHER || state.publisher.sending.is_some() {
        return None;
    }
    let p = &mut state.publisher;
    match (p.focus, code) {
        (FOCUS_PATH, KEY_ENTER) => return Some(super::click::read(state)),
        (_, KEY_ENTER) => return Some(EventOutcome::Idle),
        (FOCUS_AMOUNT, KEY_BACKSPACE) => p.amount.backspace(),
        (FOCUS_AMOUNT, 0x2E) => p.amount.start_point(),
        (FOCUS_AMOUNT, c @ 0x30..=0x39) => p.amount.digit((c - 0x30) as u8, NOX_DECIMALS),
        (FOCUS_AMOUNT, _) => return None,
        (_, KEY_BACKSPACE) => {
            text(p).0.pop();
        }
        (_, c) if (0x21..0x7F).contains(&c) => match text(p) {
            (t, max) if t.len() < max => t.push(c as u8 as char),
            _ => return Some(EventOutcome::Idle),
        },
        _ => return None,
    }
    state.failure = None;
    Some(EventOutcome::Repaint)
}
