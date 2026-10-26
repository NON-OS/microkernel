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

//! The steps that start, hold and enter, under PENDING or the volume.

use super::program::WRITER;
use super::world::{Knob, Ph, Vol, World, STUCK};

impl World {
    pub(super) fn begin() -> World {
        let ph = [Ph::Run, Ph::Run, Ph::Run];
        World { pc: [0; 3], ph, lock: None, start: 0, run: Vec::new(), vol: Vol::Free, bad: 0 }
    }

    pub(super) fn extend(&mut self, a: usize, at: u64) {
        if self.run.is_empty() {
            self.start = at;
        }
        self.run.push((a, at));
        self.pc[a] += 1;
    }

    /// A writer leaves, giving or swept: the volume's let_go drops what it
    /// left held, which needs PENDING.
    pub(super) fn let_go(&mut self, k: Knob) {
        if k == Knob::NoLetGo {
            return;
        }
        if self.lock.is_some() {
            self.bad |= STUCK;
        } else if k != Knob::NoDropHeld {
            self.run.clear();
        }
    }

    /// In when the volume lets this caller in, sweeping a dead writer first.
    pub(super) fn enter(&mut self, a: usize, k: Knob) -> Option<()> {
        if let Vol::Writer(o) = self.vol {
            if self.ph[o] != Ph::Dead {
                return None;
            }
            self.let_go(k);
            self.vol = Vol::Free;
        }
        self.vol = match (self.vol, WRITER[a]) {
            (Vol::Free, true) => Vol::Writer(a),
            (Vol::Free, false) => Vol::Readers(1),
            (Vol::Readers(n), false) => Vol::Readers(n + 1),
            _ => return None,
        };
        self.pc[a] += 1;
        Some(())
    }
}
