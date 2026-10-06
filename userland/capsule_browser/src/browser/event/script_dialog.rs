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

//! Saying what a page asked with `alert`, `confirm` or `prompt`.

use nonos_qjs::Dialog;

use crate::browser::omnibox::Change;
use crate::browser::state::State;

use super::dialog_line::{dialog_line, Asked};

/// How long the line stays up: longer than other notices, as it is the
/// page's own message and may be long.
const DIALOG_MS: i64 = 12_000;

/* The page was answered when it asked (dialog_line); this tells the
 * reader, in the notice bubble over the page and on the status line, what
 * it asked and what this browser answered. */
pub(super) fn take_script_dialog(state: &mut State) {
    let Some(asked) = state.engine.as_ref().and_then(|e| e.take_dialog()) else {
        return;
    };
    let kind = match asked.kind {
        Dialog::Alert => Asked::Alert,
        Dialog::Confirm => Asked::Confirm,
        Dialog::Prompt => Asked::Prompt,
    };
    let line = dialog_line(kind, &asked.text, asked.count);
    state.status = line.clone();
    state.ui.notice = Some((line, nonos_libc::mk_uptime_ms() + DIALOG_MS));
    state.mark(Change::Bubble);
}
