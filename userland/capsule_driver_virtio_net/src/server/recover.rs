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

//! A device that stopped, brought back (plan/50 C1): a transmit ring that
//! stayed full with the link up ends in a reset with a named state, never a
//! stuck slot. Drivers are not restarted by init, so the capsule does it.

use nonos_libc::{bring_up, mk_debug, mk_exit, EXIT_GAVE_UP};

use crate::setup::{self, Driver};

const SAY: &[u8] = b"driver.virtio_net0: transmit stalled; resetting the device\n";

/// Reset the device, give it back, and bring it up again with the station
/// address it had. The capsule leaves, as when it never came up, if it does
/// not come back.
pub fn recover(driver: &mut Driver) {
    let _ = mk_debug(SAY.as_ptr(), SAY.len());
    let mac = driver.mac;
    let _ = driver.quiesce();
    let Ok(fresh) = bring_up(b"driver.virtio_net0", setup::run) else {
        mk_exit(EXIT_GAVE_UP);
    };
    *driver = fresh;
    driver.mac = mac;
}
