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

//! Start and stop, as Linux's intel8x0 triggers them. Start resets the box
//! (which also clears its list address), points it at the list, marks every
//! entry valid but the last, clears the status and runs it. Stop clears the
//! run bit, waits for the engine to say it halted, then resets the box.

use crate::constants::bus::{CR_RPBM, CR_RR, PO_BDBAR, PO_CR, PO_LVI, PO_SR, SR_CLEAR, SR_DCH};
use crate::constants::ring::LVI_LAG;
use crate::regs::Port;
use crate::wait::until;

/// The box reset and the halt take a few bit clocks; ten milliseconds is
/// far past either.
const RESET_MS: u64 = 10;
const HALT_MS: u64 = 10;

fn reset_box(bus: &Port) -> bool {
    bus.write8(PO_CR, CR_RR);
    until(RESET_MS, || bus.read8(PO_CR) & CR_RR == 0)
}

/// False when the box did not come out of its reset.
pub fn start(bus: &Port, bdl_dev: u32) -> bool {
    if !reset_box(bus) {
        return false;
    }
    bus.write32(PO_BDBAR, bdl_dev);
    bus.write8(PO_LVI, LVI_LAG);
    bus.write16(PO_SR, SR_CLEAR);
    bus.write8(PO_CR, CR_RPBM);
    true
}

/// False when the engine did not halt or the box did not reset.
pub fn stop(bus: &Port) -> bool {
    bus.write8(PO_CR, 0);
    let halted = until(HALT_MS, || bus.read16(PO_SR) & SR_DCH != 0);
    reset_box(bus) && halted
}
