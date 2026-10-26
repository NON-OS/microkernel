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

//! The record a link goes into, as record_with_room finds or makes it.

use super::disk::R0;
use super::exec::Run;
use super::kinds::Dir;
use super::ops::{self, REC};

impl Run<'_> {
    /// record_with_room: the first record with room, else a new one,
    /// made first (ensure_record) or chained after the last (append).
    pub(super) fn room(&mut self) -> bool {
        if self.first == 0 {
            let done = self.steps(&ops::ensure());
            self.rec = self.slot[REC as usize];
            return done;
        }
        let mut r = self.first;
        while self.dir == Dir::Full && r == R0 {
            let next = self.disk.points(r).into_iter().find(|x| self.disk.recs.contains(x));
            let Some(next) = next else {
                self.after = r;
                let done = self.steps(&ops::append());
                self.rec = self.slot[REC as usize];
                return done;
            };
            r = next;
        }
        self.rec = r;
        true
    }
}
