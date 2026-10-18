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

//! The transmit watchdog. A frame is answered once it is queued, so a
//! transmitter that stops completing shows only as a ring that stays full
//! and every later send answered E_AGAIN; a driver is not respawned, so that
//! would last until reboot. Frames held while the link is down are not a
//! hang (the part sends them when it returns), so the clock runs only while
//! the link is up, frames are in flight, and none completes.

use nonos_libc::Deadline;

use crate::constants::ctrl::STATUS_LU;
use crate::constants::regs::REG_STATUS;
use crate::constants::timeouts::TX_HANG_MS;
use crate::init::recover;
use crate::log::Line;
use crate::setup::Driver;

/// After a `reclaim`, told whether it moved `clean`. True when the part was
/// found hung and reset; the line it logs names how that went.
pub fn check(driver: &mut Driver, advanced: bool) -> bool {
    let in_flight = driver.tx.clean != driver.tx.tail;
    // The link is read last, so a send that settles it otherwise pays no
    // register read for it.
    // SAFETY: `driver.regs` carries a valid broker MmioMap base for BAR0 and
    // REG_STATUS is a 32-bit register inside it (igc_regs.h).
    let link_up = || unsafe { driver.regs.r32(REG_STATUS) } & STATUS_LU != 0;
    if advanced || !in_flight || !link_up() {
        driver.tx_watch = Deadline::after_ms(TX_HANG_MS);
        return false;
    }
    if !driver.tx_watch.expired() {
        return false;
    }
    // A reset that fails is tried again once the watchdog falls due again.
    let outcome = recover(driver).err().unwrap_or("done");
    Line::new().text("tx hang, reset: ").text(outcome).send();
    driver.tx_watch = Deadline::after_ms(TX_HANG_MS);
    true
}
