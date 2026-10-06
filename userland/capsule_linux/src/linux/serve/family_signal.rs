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

//! Signals between processes of the family, and the timers that raise them,
//! settled after every answer: the outbox is routed, due timers fire, and
//! each process's parked threads take what now reaches them.

use super::family::Family;
use super::family_signal_fire::fire;
use crate::linux::call::now_ms;

const CLOCK_MONOTONIC: u64 = 1;

impl Family {
    pub(super) fn settle_signals(&mut self) {
        self.route_outbox();
        if let Some(now) = now_ms(CLOCK_MONOTONIC) {
            self.guests.iter_mut().for_each(|g| fire(g, now));
        }
        self.guests.iter_mut().for_each(super::deliver_wait::settle);
    }
}
