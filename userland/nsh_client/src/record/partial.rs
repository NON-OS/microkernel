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

//! The entries of a record that has no footer, as a crash leaves it. They
//! are read while each one parses, counts on from the one before and does
//! not go back in time; the first that does not ends the walk. Nothing
//! read here counts as checked unless a checkpoint covers it.

use nsh_wire::record::MAX_RECORD_ENTRIES;
use nsh_wire::{Entry, Reader};

pub(crate) struct Entries<'a> {
    r: Reader<'a>,
    next: u32,
    last_ms: u64,
    done: bool,
}

impl<'a> Entries<'a> {
    pub(crate) fn new(r: Reader<'a>, start_ms: u64) -> Self {
        Entries { r, next: 0, last_ms: start_ms, done: false }
    }
}

impl<'a> Iterator for Entries<'a> {
    type Item = Entry<'a>;

    fn next(&mut self) -> Option<Entry<'a>> {
        if self.done || self.r.left() == 0 || self.next == MAX_RECORD_ENTRIES {
            return None;
        }
        match Entry::read(&mut self.r) {
            Ok(e) if e.seq == self.next && e.at_ms >= self.last_ms => {
                (self.next, self.last_ms) = (self.next + 1, e.at_ms);
                Some(e)
            }
            _ => {
                self.done = true;
                None
            }
        }
    }
}
