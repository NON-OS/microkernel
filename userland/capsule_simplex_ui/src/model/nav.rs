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

//! Moving through the chat list, filtered by the search when one is open.

use alloc::vec::Vec;

use super::focus::Focus;
use super::state::Model;

fn matches(name: &str, query: &str) -> bool {
    let name = name.to_lowercase();
    query.split_whitespace().all(|word| name.contains(&word.to_lowercase()))
}

impl Model {
    /// The indexes of the chats the list shows, in order.
    pub fn visible(&self) -> Vec<usize> {
        let searching = self.focus == Focus::Search && !self.search.is_empty();
        (0..self.chats.len())
            .filter(|&i| !searching || matches(&self.chats[i].name, &self.search))
            .collect()
    }

    /// Selects the chat `delta` rows away from the selected one in the list.
    pub fn step(&mut self, delta: i32) -> bool {
        let shown = self.visible();
        if shown.is_empty() {
            return false;
        }
        let here = self.selected.and_then(|c| self.find(c));
        let pos = here.and_then(|i| shown.iter().position(|&s| s == i));
        let last = shown.len() as i32 - 1;
        let next = match pos {
            Some(p) => (p as i32 + delta).clamp(0, last),
            None => 0,
        } as usize;
        if pos == Some(next) {
            return false;
        }
        self.select(shown[next]);
        true
    }

    /// Opens the chat at index `at`.
    pub fn select(&mut self, at: usize) {
        self.selected = self.chats.get(at).map(|c| c.chat);
        self.open_selected();
    }

    /// After the search changed: when the selection no longer matches, the
    /// first match becomes it.
    pub(super) fn follow_search(&mut self) {
        let shown = self.visible();
        let here = self.selected.and_then(|c| self.find(c));
        if !here.is_some_and(|h| shown.contains(&h)) {
            if let Some(&first) = shown.first() {
                self.select(first);
            }
        }
    }
}
