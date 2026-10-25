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

//! Which keys the hint bar names, for what the reader is doing now.
//!
//! One fixed row said `^K kill` after Ctrl+K had become the palette. Each set
//! names keys bound in that state, most useful first; the bar draws what fits.

/// What the keys are for at this moment.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Typing at the prompt, searching the history, finding in the
    /// scrollback, or a program in the foreground has the keyboard.
    Prompt,
    Search,
    Find,
    Running,
}

pub type Hint = (&'static str, &'static str);

pub const PROMPT: &[Hint] = &[
    ("Tab", "complete"),
    ("Ctrl+R", "history"),
    ("Ctrl+K", "commands"),
    ("Ctrl+Shift+F", "find"),
    ("Ctrl+Shift+T", "new tab"),
    ("Ctrl+L", "clear"),
    ("help keys", "all keys"),
];

pub const SEARCH: &[Hint] =
    &[("Enter", "run"), ("Ctrl+R", "older"), ("Esc", "back to your line"), ("Bksp", "edit")];

pub const FIND: &[Hint] =
    &[("Enter", "older"), ("Shift+Enter", "newer"), ("Alt+C", "case"), ("Esc", "close")];

pub const RUNNING: &[Hint] =
    &[("Ctrl+C", "stop"), ("Ctrl+Shift+V", "paste"), ("Ctrl+Shift+C", "copy")];

/// The mode, from the state's flags. A running program outranks the rest: the
/// keys are its own, and a search or the find bar cannot be open over it.
pub fn mode(running: bool, finding: bool, searching: bool) -> Mode {
    if running {
        Mode::Running
    } else if finding {
        Mode::Find
    } else if searching {
        Mode::Search
    } else {
        Mode::Prompt
    }
}

pub fn hints(m: Mode) -> &'static [Hint] {
    match m {
        Mode::Prompt => PROMPT,
        Mode::Search => SEARCH,
        Mode::Find => FIND,
        Mode::Running => RUNNING,
    }
}
