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
//! How a request ends besides its answer: the key quiet too long, its
//! client gone, and the answer taken once. A `true` from the first two
//! tells the driver the channel is spent.

use alloc::vec::Vec;

use super::job::{Fault, Job, E_AGAIN};
use super::talk::{E_INVAL, E_TIMEDOUT};

impl Job {
    /// The key said nothing for too long: whether a request was running.
    pub fn give_up(&mut self) -> bool {
        let running = self.busy();
        if running {
            *self = Job::Done(Err((E_TIMEDOUT, None)));
        }
        running
    }

    /// The client left: its request or answer let go; whether one ran.
    pub fn abandon(&mut self) -> bool {
        let running = self.busy();
        *self = Job::Idle;
        running
    }

    /// The answer, once; -11 with the keep-alive status while it runs.
    pub fn result(&mut self) -> Result<Vec<u8>, Fault> {
        match self {
            Job::Running(_, s) => Err((E_AGAIN, Some(*s))),
            Job::Done(_) => match core::mem::take(self) {
                Job::Done(r) => r,
                _ => Err((E_INVAL, None)),
            },
            _ => Err((E_INVAL, None)),
        }
    }
}
