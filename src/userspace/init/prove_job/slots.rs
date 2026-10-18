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

//! The queue's state, apart from spawning: one PROVE and one
//! DELIVER may wait; PROVE is taken first; a DELIVER is held to the rate.

use super::ask::ProveAsk;
use super::rate::Rate;

/// The next job to spawn, with the pid it is bound to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Next {
    Prove(u32),
    Deliver(u32),
}

pub(crate) struct Slots {
    prove: Option<u32>,
    deliver: Option<u32>,
    rate: Rate,
}

impl Slots {
    pub(crate) const fn new() -> Slots {
        Slots { prove: None, deliver: None, rate: Rate::new() }
    }

    /// False when one of its kind waits, or the measurement's rate holds.
    pub(crate) fn ask(&mut self, ask: ProveAsk, now: u64) -> bool {
        match ask {
            ProveAsk::Prove(pid) if self.prove.is_none() => self.prove = Some(pid),
            ProveAsk::Deliver(pid, m) if self.deliver.is_none() && self.rate.take(&m, now) => {
                self.deliver = Some(pid)
            }
            _ => return false,
        }
        true
    }

    pub(crate) fn waiting(&self) -> bool {
        self.prove.is_some() || self.deliver.is_some()
    }

    /// PROVE first, then DELIVER.
    pub(crate) fn next(&mut self) -> Option<Next> {
        let prove = self.prove.take().map(Next::Prove);
        prove.or_else(|| self.deliver.take().map(Next::Deliver))
    }
}
