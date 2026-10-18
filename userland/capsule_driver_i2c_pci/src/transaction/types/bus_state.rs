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

//! Where the bus stands between transfers: every failure ends
//! in a state with a name, and a run of them ends in a reset.

/// Transfers in a row that may end with the bus busy or timed out before
/// the controller is reset. One is a device stretching its clock or a lost
/// arbitration; a second in a row is a bus that has stopped.
pub const RECOVER_AFTER: u32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BusState {
    /// The last transfer ended without a bus fault (a NACK is the device
    /// answering, not a fault).
    Ready,
    /// This many transfers in a row ended busy or timed out.
    Faulting(u32),
    /// The controller was just reset; the next transfer tells whether that
    /// freed the bus.
    Reset,
    /// A reset did not bring the bus back, and this many transfers have
    /// failed since. Each call still ends at its deadline, and the reset is
    /// tried again every RECOVER_AFTER failures.
    Stuck(u32),
}

impl BusState {
    /// The console line a change from `was` to `now` is worth, if any: one
    /// per incident, not one per fault.
    pub fn news(was: BusState, now: BusState) -> Option<&'static [u8]> {
        use BusState::{Faulting, Ready, Reset, Stuck};
        let line: &'static [u8] = match (was, now) {
            (Faulting(_), Reset) => b"driver.i2c_pci: the I2C bus stopped; controller reset\n",
            (Reset | Stuck(_), Ready) => b"driver.i2c_pci: the I2C bus is moving again\n",
            (Ready | Faulting(_) | Reset, Stuck(_)) => {
                b"driver.i2c_pci: the I2C bus is still stuck after a reset; a device may hold it\n"
            }
            _ => return None,
        };
        Some(line)
    }
}
