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

//! The tab strip and the builders that set it up.

use crate::icons::IconId;
use crate::widgets::State;

/// A row of tabs, one per label, with one selected.
#[derive(Clone, Copy, Debug)]
pub struct Tabs<'a> {
    pub(super) labels: &'a [&'a str],
    pub(super) icons: &'a [IconId],
    pub(super) busy: &'a [bool],
    pub(super) selected: usize,
    pub(super) hovered: Option<usize>,
    pub(super) state: State,
}

impl<'a> Tabs<'a> {
    /// A strip of `labels` with the tab at `selected` chosen. An index past
    /// the end selects nothing.
    pub fn new(labels: &'a [&'a str], selected: usize) -> Self {
        Tabs { labels, icons: &[], busy: &[], selected, hovered: None, state: State::default() }
    }

    /// The icon each tab leads with, by index: an `ICON` square `SPACE.sm`
    /// before the label, in the label's ink. Tabs past the end have none.
    pub fn icons(mut self, icons: &'a [IconId]) -> Self {
        self.icons = icons;
        self
    }

    /// Which tabs are busy, by index: a busy tab shows a `DOT` in the accent
    /// in its icon's place, so its width does not move while it loads if it
    /// has an icon. Tabs past the end are not busy.
    pub fn busy(mut self, busy: &'a [bool]) -> Self {
        self.busy = busy;
        self
    }

    /// The tab under the pointer, if any. It gets a hover wash unless it is
    /// the selected one.
    pub fn hovered(mut self, index: Option<usize>) -> Self {
        self.hovered = index;
        self
    }

    /// The state the app reports for the whole strip. Only `disabled` and
    /// `focused` are read.
    pub fn state(mut self, s: State) -> Self {
        self.state = s;
        self
    }
}
