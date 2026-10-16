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

//! A part on the clock (`on_clock`) holds a wait's second look until uptime
//! reads the bound past its deadline's count, where a deadline a millisecond
//! short falls due and a whole one does not; spinning there on its own, a
//! wait lands a look in that millisecond only when the host lets it.

use std::{cell::Cell, rc::Rc};

use nonos_libc::{on_clock, uptime_ms};

use super::clock_edge::{land_on, late_in_a_millisecond};

/// Runs before a host that never wakes the held look in time is given up on.
const TRIES: u32 = 20;

/// The bound's count, whether the held look landed as it began, and the
/// looks from that one on.
#[derive(Default)]
struct Looks {
    bound: Cell<Option<u64>>,
    landed: Cell<Option<bool>>,
    from_bound: Cell<u32>,
}

impl Looks {
    fn look(&self, ms: u64) {
        let Some(bound) = self.bound.get() else {
            return self.bound.set(Some(uptime_ms() + ms));
        };
        if self.landed.get().is_none() {
            self.landed.set(Some(land_on(bound)));
        }
        self.from_bound.set(self.from_bound.get() + 1);
    }
}

/// `wait`, begun late in a millisecond, run until its look held to the
/// bound of `ms` lands as that millisecond begins: a wait that looks no
/// more after it gave up a millisecond short. A host that never lands it
/// leaves the elapsed-time check `wait` makes on every run.
pub fn never_gives_up_at_the_bound(ms: u64, wait: impl Fn()) {
    for _ in 0..TRIES {
        let looks = Rc::new(Looks::default());
        let part = Rc::clone(&looks);
        let _clock = on_clock(move || part.look(ms));
        late_in_a_millisecond();
        wait();
        if looks.landed.get() == Some(true) {
            let more = looks.from_bound.get() > 1;
            return assert!(more, "gave up at the look {ms} ms on, a millisecond short");
        }
    }
}
