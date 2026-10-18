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

//! Giving a controller up (xHCI 1.2 sections 4.2 and 5.4.1). Its DMA is
//! freed in one order only, whatever order `Driver`'s fields are declared
//! in: `Drop::drop` runs before any field is dropped, and it stops the
//! controller at its source (RUN cleared, then HCH within `halt`'s
//! deadline, then HCRST so it forgets every pointer it was given), turns
//! bus mastering off, and releases the device. Only then do the fields,
//! the DCBAA, scratchpads, rings and slot contexts, drop.

use super::driver::Driver;
use crate::controller::{halt, reset};

/// How far a controller could be stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quiesced {
    /// Halted and reset: it holds no pointer the driver gave it.
    Reset,
    /// Halted, but the reset never finished: it runs no ring.
    Halted,
    /// It never said halted, so it is not reset (HCRST on a running
    /// controller is undefined); bus mastering off and the release are all
    /// that keep it from memory.
    Running,
}

/// Stop the controller whose operational registers are at `op_base`.
pub fn quiesce(op_base: u64) -> Quiesced {
    if halt(op_base).is_err() {
        return Quiesced::Running;
    }
    match reset(op_base) {
        Ok(()) => Quiesced::Reset,
        Err(_) => Quiesced::Halted,
    }
}

impl Drop for Driver {
    fn drop(&mut self) {
        let _ = quiesce(self.layout.op_base);
        let _ = self.handles.bus_master_off();
        self.handles.release();
    }
}
