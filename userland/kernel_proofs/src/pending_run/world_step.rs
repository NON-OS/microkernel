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

//! One step of one caller. Only a send puts a caller off its CPU, so only
//! there can it sleep, be killed (H alone), or keep PENDING held (the old
//! hold); every other step is whole under PENDING or the volume.

use super::pending_rule::must_send;
use super::program::{Op, CAP, H, PROGRAM};
use super::world::{Knob, Ph, World, STUCK};

impl World {
    pub(super) fn done(&self, a: usize) -> bool {
        self.ph[a] == Ph::Dead || self.pc[a] == PROGRAM[a].len()
    }

    /// Every world one step of one caller leads to; H may also give up
    /// part-way, as an error path does, or die asleep in a send.
    pub(super) fn next(&self, k: Knob) -> Vec<World> {
        let mut out = Vec::new();
        for a in (0..3).filter(|&a| !self.done(a)) {
            if let Ph::Asleep { .. } = self.ph[a] {
                out.push(self.woken(a));
                if a == H {
                    let mut w = self.clone();
                    w.ph[a] = Ph::Dead;
                    out.push(w);
                }
                continue;
            }
            out.extend(self.op(a, k));
            if a == H && (2..PROGRAM[H].len() - 1).contains(&self.pc[a]) {
                let mut w = self.clone();
                w.pc[a] = PROGRAM[H].len() - 1;
                out.push(w);
            }
        }
        out
    }

    fn op(&self, a: usize, k: Knob) -> Option<World> {
        let (mut w, op) = (self.clone(), PROGRAM[a][self.pc[a]]);
        if matches!(op, Op::Hold(_) | Op::Flush) && self.lock.is_some_and(|o| o != a) {
            w.bad |= STUCK;
            return Some(w);
        }
        match op {
            Op::Enter => w.enter(a, k)?,
            Op::Hold(at) if must_send(w.start, w.run.len() as u64, at, CAP) => w.send(a, k, false),
            Op::Hold(at) => w.extend(a, at),
            Op::Flush if w.run.is_empty() => w.pc[a] += 1,
            Op::Flush => w.send(a, k, true),
            Op::Give => w.leave_writer(a, k),
            Op::Read(lba) => w.read(a, lba),
            Op::Leave => w.leave_reader(a),
        }
        Some(w)
    }
}
