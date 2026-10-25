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

//! What the query matches: a name or a description holding it.

use super::Search;

impl Search {
    /// A listing is kept when its name or, once known, its description
    /// holds the query.
    pub fn matches(&self, name: &[u8], description: Option<&[u8]>) -> bool {
        self.accepts(name) || description.is_some_and(|d| self.accepts(d))
    }

    /// Case-insensitive substring. Empty accepts everything; longer
    /// than the name matches nothing rather than panicking.
    pub fn accepts(&self, name: &[u8]) -> bool {
        if self.text.is_empty() {
            return true;
        }
        if self.text.len() > name.len() {
            return false;
        }
        let lower = |b: &u8| b.to_ascii_lowercase();
        name.windows(self.text.len()).any(|w| w.iter().map(lower).eq(self.text.iter().copied()))
    }
}
