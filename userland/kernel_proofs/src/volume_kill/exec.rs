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

//! One caller running one operation on the disk and the shared mount, to
//! its end or to its `stop`th request, which is then left in flight.

use super::disk::{Disk, D};
use super::kinds::{Dir, Mount, Op, Write};
use super::ops;
use super::ops_more;
use super::order::Step;

pub(super) struct Run<'a> {
    pub(super) disk: &'a mut Disk,
    pub(super) mount: &'a mut Mount,
    pub(super) dir: Dir,
    pub(super) stop: Option<usize>,
    pub(super) sent: usize,
    pub(super) flight: Option<Write>,
    pub(super) bad: u8,
    /// Its blocks by slot, its directory's first record as it read it, the
    /// record it links into, and the record a new one is chained after.
    pub(super) slot: [u64; 4],
    pub(super) first: u64,
    pub(super) rec: u64,
    pub(super) after: u64,
}

impl Run<'_> {
    pub(super) fn op(&mut self, op: Op) {
        self.first = self.disk.points(D).into_iter().next().unwrap_or(0);
        let steps = match op {
            Op::Create => ops::create(),
            Op::Unlink => ops_more::unlink(),
            Op::Close => ops_more::close(),
        };
        self.steps(&steps);
    }

    /// Every step until one stops the operation.
    pub(super) fn steps(&mut self, steps: &[Step]) -> bool {
        steps.iter().all(|&s| self.step(s))
    }
}
