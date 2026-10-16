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

//! The end of a GMBUS transaction: a failed one has the controller's
//! interrupt cleared first; then the stop, the wait for the bus to go
//! idle (with a deadline), and the pins let go.

use super::super::access::{Clock, Expiry, Mmio, Until};
use super::super::gmbus_regs::*;

/// The bus going idle after the stop.
const IDLE_US: u64 = 50_000;

pub fn end(m: &mut dyn Mmio, clock: &dyn Clock, failed: bool) {
    if failed {
        m.write(GMBUS1, SW_CLR_INT);
        m.write(GMBUS1, 0);
    }
    m.write(GMBUS1, SW_RDY | CYCLE_STOP);
    let until = Until::new(clock, IDLE_US);
    while m.read(GMBUS2) & ACTIVE != 0 && !until.expired() {}
    m.write(GMBUS0, 0);
}
