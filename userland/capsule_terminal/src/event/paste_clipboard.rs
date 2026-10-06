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

use nonos_app_skeleton::{clipboard_paste_line, EventOutcome};

use crate::term::dimensions::LINE_MAX;
use crate::term::state::State;

pub fn paste_clipboard(state: &mut State) -> EventOutcome {
    // One byte past what the line holds, so a line too long to fit is seen
    // to be, rather than read in cut to exactly the space there is.
    let mut buf = [0u8; LINE_MAX + 1];
    // Stop at the first newline rather than skipping over it. Dropping
    // newlines glued separate commands into one line, which the next Enter
    // then ran as a single mangled command.
    let line = match clipboard_paste_line(&mut buf) {
        Ok(Some(line)) => line,
        Ok(None) => {
            state.scrollback.push_line(b"paste: the clipboard does not hold text");
            return EventOutcome::Repaint;
        }
        Err(_) => return EventOutcome::Idle,
    };
    let pasted = state.line.paste(line.text);
    // The input line holds LINE_MAX bytes, so anything longer cannot fit. Say so
    // instead of leaving a silently shortened command on the prompt.
    if line.more {
        state.scrollback.push_line(b"paste: first line only");
    } else if pasted.full {
        state.scrollback.push_line(b"paste: line full, clipboard truncated");
    }
    if !pasted.changed {
        return EventOutcome::Repaint;
    }
    state.history.reset_cursor();
    state.scrollback.jump_bottom();
    EventOutcome::Repaint
}
