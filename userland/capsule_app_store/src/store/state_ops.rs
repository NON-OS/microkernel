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

//! Moving through the catalogue.

use alloc::vec::Vec;

use super::state::{State, Tab};

impl State {
    pub fn new() -> State {
        let mut state = State {
            listings: Vec::new(),
            tab: Tab::All,
            cursor: 0,
            scroll: 0,
            rows: 1,
            fb_w: 0,
            fb_h: 0,
            trouble: None,
            ready: None,
            asked: None,
            search: super::search::Search::default(),
            detail: None,
        };
        state.refresh();
        state
    }

    /// Indices into `listings` that the current tab shows.
    pub fn visible(&self) -> Vec<usize> {
        let keep =
            |(i, l): (usize, &super::listing::Listing)| self.tab.accepts(l.source).then_some(i);
        self.listings.iter().enumerate().filter_map(keep).collect()
    }
}
