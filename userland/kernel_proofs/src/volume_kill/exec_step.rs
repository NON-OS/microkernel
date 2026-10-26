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

//! One step, as the kernel takes it: an allocation and a commit move the
//! shared mount; every write is one request to the device.

use super::disk::{D, N};
use super::exec::Run;
use super::kinds::Write;
use super::ops::{self, DATA, INDEX, REC};
use super::order::Step::{self, *};

impl Run<'_> {
    pub(super) fn step(&mut self, s: Step) -> bool {
        let block = |lba, to, rec| Write::Block { lba, to, rec };
        match s {
            Alloc(k) => self.alloc(k),
            Commit => {
                self.mount.generation += 1;
                let (generation, free) = (self.mount.generation, self.mount.free);
                self.send(Write::Header { generation, free })
            }
            Init(k) => {
                let to = if k == INDEX { [self.slot[DATA as usize]].into() } else { [].into() };
                self.send(block(self.slot[k as usize], to, k == REC))
            }
            Link(c) => self.steps(&ops::link(c)),
            Room => self.room(),
            PointRec(c) => self.add(self.rec, self.slot[c as usize]),
            WriteDir => {
                let first = (self.first != 0).then_some(self.first);
                self.send(block(D, first.into_iter().collect(), false))
            }
            FirstRec(k) => {
                self.first = self.slot[k as usize];
                true
            }
            Chain(k) => self.add(self.after, self.slot[k as usize]),
            UnpointRec => self.unpoint(),
            PointNode(k) => self.send(block(N, [self.slot[k as usize]].into(), false)),
            Tree => {
                let data = self.alloc(DATA).then(|| self.slot[DATA as usize]);
                data.is_some_and(|lba| self.send(block(lba, [].into(), false)))
            }
        }
    }
}
