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

//! A request the device never answered, brought to an end:
//! the device is reset with it still posted, given back, and brought up
//! again on fresh grants. Its buffer is never read and no request is
//! posted over it, so later requests do not each wait it out, as they did
//! when every one spent its whole budget waiting for the device to catch
//! up. The log names the state ended, and whether the device itself asked
//! for the reset (DEVICE_NEEDS_RESET). Drivers are not restarted by init,
//! so the capsule does it, and leaves, as when it never came up, if the
//! device does not come back, or has been reset too often (`streak`).

use nonos_libc::{bring_up, mk_debug, mk_exit, EXIT_GAVE_UP};

use super::streak::{RESETS_IN_A_ROW, SERVED_SPAN_MS};
use crate::queue::Fault;
use crate::setup::{self, Driver};

pub fn recover(driver: &mut Driver, why: Fault) {
    say(driver, why, "resetting the device");
    let _ = driver.quiesce();
    let Ok(fresh) = bring_up(b"driver.virtio_blk0", setup::run) else {
        mk_exit(EXIT_GAVE_UP);
    };
    *driver = fresh;
}

/// The stall after `RESETS_IN_A_ROW` resets: the device is stopped and
/// given back, and the driver leaves, so the disk reads as gone.
pub fn give_up(driver: &Driver, why: Fault) -> ! {
    let span = SERVED_SPAN_MS / 1000;
    let gone = alloc::format!(
        "{RESETS_IN_A_ROW} resets in a row with under {span} s served between; the disk is given up"
    );
    say(driver, why, &gone);
    let _ = driver.quiesce();
    mk_exit(EXIT_GAVE_UP);
}

fn say(driver: &Driver, why: Fault, then: &str) {
    let asked = if driver.transport.needs_reset() { "; the device asked for it" } else { "" };
    let line = alloc::format!("driver.virtio_blk0: {}{asked}; {then}\n", why.name());
    let _ = mk_debug(line.as_ptr(), line.len());
}
