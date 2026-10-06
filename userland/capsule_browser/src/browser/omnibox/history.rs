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

use alloc::string::String;
use alloc::vec::Vec;

/* Most entries session history keeps; the oldest is dropped past it. */
pub const HISTORY_CAP: usize = 100;

/* Session history: the visited addresses and the one being shown. `index`
 * is meaningful only while `entries` is not empty. */
#[derive(Clone, Debug, Default)]
pub struct History {
    pub entries: Vec<String>,
    pub index: usize,
}

impl History {
    pub fn new() -> Self {
        History::default()
    }

    pub fn current(&self) -> Option<&str> {
        self.entries.get(self.index).map(String::as_str)
    }

    /* A new visit: forward entries are dropped, and loading the address
     * already shown adds nothing. */
    pub fn push(&mut self, url: &str) {
        if self.current() == Some(url) {
            return;
        }
        if !self.entries.is_empty() {
            self.entries.truncate(self.index + 1);
        }
        self.entries.push(String::from(url));
        if self.entries.len() > HISTORY_CAP {
            self.entries.remove(0);
        }
        self.index = self.entries.len() - 1;
    }

    /* A reload, a redirect or a back/forward landing rewrites the entry in
     * place instead of adding one. */
    pub fn replace(&mut self, url: &str) {
        match self.entries.get_mut(self.index) {
            Some(e) => *e = String::from(url),
            None => self.push(url),
        }
    }
}
