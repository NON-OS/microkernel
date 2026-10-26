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

//! Finding an entry by name, and the entries under a directory, in the
//! sorted index.

use core::ops::Range;

use crate::entry::Entry;
use crate::index::Index;

impl<'a> Index<'a> {
    pub fn len(&self) -> usize {
        self.header.count
    }

    pub fn is_empty(&self) -> bool {
        self.header.count == 0
    }

    /// The entry named `name`, by binary search: the index is sorted.
    pub fn find(&self, name: &[u8]) -> Option<Entry<'a>> {
        let i = self.first_at_least(name);
        self.entry(i).ok().filter(|e| e.name == name)
    }

    /// The indices of every entry below `dir` (`dir/...`), in order.
    pub fn under(&self, dir: &[u8]) -> Range<usize> {
        let mut lo = [dir, b"/"].concat();
        let start = self.first_at_least(&lo);
        // '0' follows '/': the first name past every `dir/...`.
        *lo.last_mut().unwrap() = b'0';
        start..self.first_at_least(&lo)
    }

    fn first_at_least(&self, name: &[u8]) -> usize {
        let (mut lo, mut hi) = (0, self.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match self.entry(mid) {
                Ok(e) if e.name < name => lo = mid + 1,
                _ => hi = mid,
            }
        }
        lo
    }
}
