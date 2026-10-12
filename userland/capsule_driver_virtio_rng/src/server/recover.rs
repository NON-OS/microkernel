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

//! A request the device never answered, brought to an end: the
//! device is reset with it still posted, given back, and brought up again
//! on fresh grants. Its buffer is never read, and no request is posted over
//! it. Drivers are not restarted by init, so the capsule does it.

use nonos_libc::{bring_up, mk_debug, mk_exit, EXIT_GAVE_UP};

use crate::setup::{self, Driver};

const SAY: &[u8] = b"driver.virtio_rng: request stalled; resetting the device\n";

/// Reset the device, give it back, and bring it up again. The capsule
/// leaves, as when it never came up, if it does not come back.
pub fn recover(driver: &mut Driver) {
    let _ = mk_debug(SAY.as_ptr(), SAY.len());
    let _ = driver.quiesce();
    let Ok(fresh) = bring_up(b"driver.virtio_rng", setup::run) else {
        mk_exit(EXIT_GAVE_UP);
    };
    *driver = fresh;
}
