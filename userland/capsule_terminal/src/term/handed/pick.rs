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

//! Which tab a handed command goes to. A Terminal the tile just opened shows
//! one untouched tab, and the command belongs there rather than in a second
//! one beside it. A Terminal in use keeps what each tab holds: the command
//! gets a tab of its own. With every tab taken, an idle prompt will do; a tab
//! with a program running or a line half typed is never written over.

/// The active tab, as far as the choice needs it.
#[derive(Clone, Copy)]
pub struct Active {
    /// Nothing has been run in it yet.
    pub fresh: bool,
    /// Its prompt is empty, no program holds it and no search is open.
    pub idle: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pick {
    Current,
    NewTab,
    /// Every tab is busy: the command is not run anywhere.
    Nowhere,
}

pub fn pick(active: Active, tabs: usize, max_tabs: usize) -> Pick {
    if active.fresh && active.idle {
        Pick::Current
    } else if tabs < max_tabs {
        Pick::NewTab
    } else if active.idle {
        Pick::Current
    } else {
        Pick::Nowhere
    }
}
