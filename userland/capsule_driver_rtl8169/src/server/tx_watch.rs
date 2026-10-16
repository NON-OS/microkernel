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
//! transmitter that stops completing shows only as a next slot that stays
//! the part's and every later send answered E_AGAIN; a driver is not
//! respawned, so that would last until reboot. A slot held while the link is
//! down is not a hang (the part sends once it returns), so the clock runs
//! only while the link is up and the next slot stays the part's.

use nonos_libc::Deadline;

use crate::constants::queue::TX_HANG_MS;
use crate::constants::regs::REG_PHY_STATUS;
use crate::init::recover;
use crate::link::decode;
use crate::log::Line;
use crate::setup::Driver;

/// Told whether the next slot is still the part's. True when the part was
/// found hung and reset; the line it logs names how that went.
pub fn check(driver: &mut Driver, busy: bool) -> bool {
    // The link is read last, so a send that finds its slot free pays no
    // register read for it.
    // SAFETY: PHYstatus (0x6C..0x6D) lies inside every mapped window.
    let link_up = || decode(unsafe { driver.regs.r16(REG_PHY_STATUS) }, driver.chip.ver).up;
    if !busy || !link_up() {
        driver.tx_watch = Deadline::after_ms(TX_HANG_MS);
        return false;
    }
    if !driver.tx_watch.expired() {
        return false;
    }
    let line = Line::new("rtl8169: tx hang, reset: ");
    match recover(driver) {
        Ok(()) => line.text("done").send(),
        // Tried again once the watchdog falls due again.
        Err(e) => line.text(e).send(),
    }
    driver.tx_watch = Deadline::after_ms(TX_HANG_MS);
    true
}
