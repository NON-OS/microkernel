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

//! The chords the input router hands the shell over whatever window has
//! focus: Ctrl+Alt+Esc, Ctrl+Alt+T and Alt+Tab.

use crate::server::handlers::escape_chord as escape;
use crate::server::handlers::switch_key;
use crate::state::Context;

/// Answer a chord's press; whether the key was one.
pub(super) fn chord(ctx: &mut Context, kind: u16, code: u32, flags: u16) -> bool {
    if escape::is_reserved_chord(kind, code, flags) {
        escape::bring_process_manager(ctx);
        return true;
    }
    // Ctrl+Alt+T and Alt+Tab are swallowed while a prompt holds the trusted
    // band, never typed into it and never moving a window over it.
    let held = !escape::opens_terminal(ctx.trusted.up);
    if escape::is_terminal_chord(kind, code, flags) {
        if !held {
            escape::bring_terminal(ctx);
        }
        return true;
    }
    if escape::is_switch_chord(kind, code, flags) {
        if !held {
            switch_key::switch(ctx, flags);
        }
        return true;
    }
    false
}
