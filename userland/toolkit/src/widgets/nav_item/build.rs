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

//! The navigation item and the builders that set it up.

use crate::icons::IconId;
use crate::widgets::State;

/// A sidebar entry with a one-line label and an optional icon before it.
#[derive(Clone, Copy, Debug)]
pub struct NavItem<'a> {
    pub(super) label: &'a str,
    pub(super) icon: Option<IconId>,
    pub(super) active: bool,
    pub(super) count: Option<u32>,
    pub(super) compact: bool,
    pub(super) state: State,
}

impl<'a> NavItem<'a> {
    /// An inactive item showing `label`, with no icon.
    pub fn new(label: &'a str) -> Self {
        let state = State::default();
        NavItem { label, icon: None, active: false, count: None, compact: false, state }
    }

    /// The icon drawn before the label, in the label's ink.
    pub fn icon(mut self, id: IconId) -> Self {
        self.icon = Some(id);
        self
    }

    /// Whether this item's page is the one on show.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// A count right-aligned at the trailing end, such as how many pages
    /// the item holds, in `muted`. It is left out when the item is too
    /// narrow to show it whole.
    pub fn count(mut self, n: u32) -> Self {
        self.count = Some(n);
        self
    }

    /// Shows only the icon, centred in the body, for a sidebar folded to a
    /// rail. The label is not drawn but stays the item's name, for the app
    /// to give a tooltip or announce. Any count is left out.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// The state the app reports. `selected` is ignored; use `active`.
    pub fn state(mut self, s: State) -> Self {
        self.state = s;
        self
    }
}
