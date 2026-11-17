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

//! The pill and the builders that set it up.

use crate::icons::IconId;
use crate::theme::Category;
use crate::widgets::{State, Tone};

/// A status pill with a one-line label.
#[derive(Clone, Copy, Debug)]
pub struct Pill<'a> {
    pub(super) label: &'a str,
    pub(super) tone: Tone,
    pub(super) dot: bool,
    pub(super) state: State,
    pub(super) count: Option<u32>,
    pub(super) category: Option<Category>,
    pub(super) filled: bool,
    pub(super) icon: Option<IconId>,
}

impl<'a> Pill<'a> {
    /// A `Plain` pill showing `label`, with no dot.
    pub fn new(label: &'a str) -> Self {
        let (tone, state) = (Tone::Plain, State::default());
        let (count, category, icon) = (None, None, None);
        Pill { label, tone, dot: false, state, count, category, filled: false, icon }
    }

    /// The status the pill shows, which colours the dot, or the label when
    /// there is no dot.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    /// Puts a status dot in the tone's colour before the label.
    pub fn dot(mut self) -> Self {
        self.dot = true;
        self
    }

    /// An `ICON` square before the label, `SPACE.xs` short of it, in the
    /// label's ink. It takes the dot's place: with both, only the icon is
    /// drawn. A filled pill keeps it.
    pub fn icon(mut self, id: IconId) -> Self {
        self.icon = Some(id);
        self
    }

    /// The state the app reports.
    pub fn state(mut self, s: State) -> Self {
        self.state = s;
        self
    }

    /// A count after the label, `SPACE.sm` past it, in `muted` digits, such
    /// as how many items a filter pill holds. The pill widens to hold it.
    pub fn count(mut self, n: u32) -> Self {
        self.count = Some(n);
        self
    }

    /// Puts a dot in the category's colour before the label, in place of
    /// the tone's, and undoes `filled`.
    pub fn category(mut self, c: Category) -> Self {
        self.category = Some(c);
        self.dot = true;
        self.filled = false;
        self
    }

    /// Fills the body in the category's colour and draws the label and any
    /// count in the ink that reads on it. A filled pill has no dot, even
    /// with `dot` or `category`. Disabled draws the plain disabled pill.
    pub fn filled(mut self, c: Category) -> Self {
        self.category = Some(c);
        self.filled = true;
        self
    }
}
