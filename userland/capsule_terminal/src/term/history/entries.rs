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

//! Letting expansion read the history.
//!
//! The expander takes a trait rather than the concrete type so it can be
//! driven from a host test with a handful of lines. This is the one
//! implementation that matters: the real ring, read back exactly as the
//! `history` command prints it, so `!2` and the second line anyone sees are
//! the same entry.

use super::expand::Entries;
use super::types::History;

impl Entries for History {
    fn count(&self) -> usize {
        self.count()
    }

    fn get(&self, index: usize) -> &[u8] {
        self.get(index)
    }
}
