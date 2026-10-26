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

//! The executor's I/O: the allocation, a block read, changed and written
//! whole, and the request that is cut.

use super::disk::N;
use super::exec::Run;
use super::kinds::Write;
use super::run::REUSE;

impl Run<'_> {
    /// A block handed out that the disk already holds or points at is a
    /// live block given twice while the machine runs.
    pub(super) fn alloc(&mut self, k: u8) -> bool {
        let lba = self.mount.free;
        let live = self.disk.blocks.contains_key(&lba)
            || self.disk.blocks.values().any(|to| to.contains(&lba));
        self.bad |= REUSE * u8::from(live);
        self.mount.free += 1;
        self.slot[k as usize] = lba;
        true
    }

    /// `lba` read and written back pointing at `to` as well.
    pub(super) fn add(&mut self, lba: u64, to: u64) -> bool {
        let mut now = self.disk.points(lba);
        now.insert(to);
        self.send(Write::Block { lba, to: now, rec: true })
    }

    /// N's entry taken out of the record that holds it, if one does.
    pub(super) fn unpoint(&mut self) -> bool {
        let holds = |r: &u64| self.disk.points(*r).contains(&N);
        let Some(r) = self.disk.recs.iter().copied().find(holds) else { return false };
        let mut now = self.disk.points(r);
        now.remove(&N);
        self.send(Write::Block { lba: r, to: now, rec: true })
    }

    /// One request: it lands, unless it is the one the caller is cut at.
    pub(super) fn send(&mut self, w: Write) -> bool {
        if self.stop == Some(self.sent) {
            self.flight = Some(w);
            return false;
        }
        self.sent += 1;
        self.disk.land(&w);
        true
    }
}
