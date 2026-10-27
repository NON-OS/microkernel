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

//! What each key does where the focus is.

use super::focus::{Focus, Key};
use super::state::Model;

impl Model {
    /// Takes one key; true when the window must repaint.
    pub fn on_key(&mut self, key: Key) -> bool {
        let f = self.focus;
        match (key, f) {
            (Key::Search, _) => {
                self.focus = Focus::Search;
                self.search.clear();
            }
            (Key::Up | Key::Down, Focus::List | Focus::Search) => {
                return self.step(if key == Key::Up { -1 } else { 1 });
            }
            (Key::Tab, Focus::Compose) => self.focus = Focus::List,
            (Key::Tab | Key::Enter, _) if f != Focus::Compose => {
                self.focus = Focus::Compose;
                self.search.clear();
            }
            (Key::Enter, _) => return self.submit(),
            (Key::Esc, Focus::List) => return false,
            (Key::Esc, _) => {
                self.focus = Focus::List;
                self.search.clear();
            }
            (Key::Backspace, Focus::Search) => {
                self.search.pop();
            }
            (Key::Backspace, _) => {
                self.compose.pop();
            }
            (Key::Char(ch), _) => {
                if f == Focus::List {
                    self.focus = Focus::Compose;
                }
                let mut buf = [0u8; 4];
                self.insert(ch.encode_utf8(&mut buf));
                if f == Focus::Search {
                    self.follow_search();
                }
            }
            _ => return false,
        }
        if f == Focus::Search && key == Key::Backspace {
            self.follow_search();
        }
        true
    }
}
