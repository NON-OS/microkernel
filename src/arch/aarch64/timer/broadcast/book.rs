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

//! Which sleeping CPUs the broadcast timer must wake, and when. Pure, so
//! the proofs run it against a model over generated sleep and wake orders.
//!
//! Each operation returns the compare value the timer must hold from now
//! on: the earliest deadline of any CPU still asleep, or `None` to leave it
//! off. The caller writes exactly that under the same lock, so the timer
//! never holds a value later than some sleeper's deadline.

/// A CPU outside the book. It is never handed a timer-stop state, since
/// nothing here could wake it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfRange;

pub struct Book<const N: usize> {
    /// Per CPU, the counter value it must be awake by; `None` while awake
    /// or with no deadline.
    due: [Option<u64>; N],
}

impl<const N: usize> Book<N> {
    pub const fn new() -> Self {
        Self { due: [None; N] }
    }

    /// CPU `cpu` stops its own timer until `due`, or with nothing to wake
    /// for when `due` is `None`.
    pub fn sleep(&mut self, cpu: usize, due: Option<u64>) -> Result<Option<u64>, OutOfRange> {
        *self.due.get_mut(cpu).ok_or(OutOfRange)? = due;
        Ok(self.earliest())
    }

    /// CPU `cpu` is awake, by the broadcast or by anything else.
    pub fn wake(&mut self, cpu: usize) -> Option<u64> {
        if let Some(slot) = self.due.get_mut(cpu) {
            *slot = None;
        }
        self.earliest()
    }

    /// The timer fired with the counter at `now`: every CPU due by then is
    /// taken out and handed to `woken`, which sends it its wake.
    pub fn expire(&mut self, now: u64, mut woken: impl FnMut(usize)) -> Option<u64> {
        for (cpu, slot) in self.due.iter_mut().enumerate() {
            if slot.is_some_and(|due| due <= now) {
                *slot = None;
                woken(cpu);
            }
        }
        self.earliest()
    }

    pub fn earliest(&self) -> Option<u64> {
        self.due.iter().flatten().copied().min()
    }
}
