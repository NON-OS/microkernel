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
//! transmitter that stops completing shows only as four slots that stay
//! full and every later send answered E_AGAIN; a driver is not respawned, so
//! that would last until reboot. Frames held while the link is down are not
//! a hang (the part sends them when it returns), so the clock runs only
//! while the link is up, frames are in flight, and none finishes.

use nonos_libc::{mk_debug, Deadline};

use crate::constants::dma::TX_HANG_MS;
use crate::constants::regs::{MSR_LINK_BAD, REG_MSR};
use crate::init::recover;
use crate::setup::Driver;

/// After a `reclaim`, told whether it moved `tx_dirty`. True when the part
/// was found hung and reset; the line it logs names how that went.
pub fn check(driver: &mut Driver, advanced: bool) -> bool {
    let in_flight = driver.tx_cur != driver.tx_dirty;
    // The link is read last, so a send that settles it otherwise pays no
    // port read for it.
    let link_up = || driver.pio.r8(REG_MSR).is_ok_and(|msr| msr & MSR_LINK_BAD == 0);
    if advanced || !in_flight || !link_up() {
        driver.tx_watch = Deadline::after_ms(TX_HANG_MS);
        return false;
    }
    if !driver.tx_watch.expired() {
        return false;
    }
    say(recover(driver));
    driver.tx_watch = Deadline::after_ms(TX_HANG_MS);
    true
}

/// One console line: that the part was reset, or the step that stopped the
/// reset, which is tried again once the watchdog falls due again.
fn say(outcome: Result<(), &'static str>) {
    let parts: [&[u8]; 2] =
        [b"driver.rtl8139: tx hang, reset: ", outcome.err().unwrap_or("done").as_bytes()];
    let mut line = [0u8; 96];
    let mut n = 0;
    for &b in parts.iter().flat_map(|p| p.iter()).take(line.len() - 1) {
        line[n] = b;
        n += 1;
    }
    line[n] = b'\n';
    let _ = mk_debug(line.as_ptr(), n + 1);
}
