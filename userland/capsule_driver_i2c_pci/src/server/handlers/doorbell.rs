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

//! The touchpad's "fresh report" doorbell: reply whether the pad's interrupt
//! line rang since the last look. An i2c-HID device holds its line asserted
//! while an input report waits (HID over I2C v1.0 section 7.4, as recalled,
//! unverified), and driver.gpio0, which owns the GPIO controller, reports it
//! on every poll while it stays asserted; the HID driver reads the input
//! register when it rang. `present` is zero when the firmware declared no
//! line or gpio0 does not answer for this device, and the HID driver then
//! reads on its timer.

use core::sync::atomic::{AtomicBool, Ordering};

use nonos_libc::mk_debug;

use crate::driver::Driver;
use crate::gpio0::look;
use crate::protocol::{Request, E_OK};
use crate::server::respond;

/// Set once the line has been seen asserted, so the first ring is said once.
static RANG: AtomicBool = AtomicBool::new(false);

pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, out: &mut [u8]) {
    let (present, fired) = match driver.doorbell.as_ref().and_then(look) {
        Some(rang) => (1u32, u32::from(rang)),
        None => (0u32, 0u32),
    };
    // The first ring proves the pad and its polarity were found right; with
    // no such line after a touch, i2c-hid is reading on its timer.
    if fired == 1 && !RANG.swap(true, Ordering::Relaxed) {
        let line = b"driver.i2c_pci: touchpad interrupt line rang for the first time\n";
        let _ = mk_debug(line.as_ptr(), line.len());
    }
    let mut body = [0u8; 8];
    body[0..4].copy_from_slice(&present.to_le_bytes());
    body[4..8].copy_from_slice(&fired.to_le_bytes());
    let _ = respond::send(sender_pid, req, E_OK, &body, out);
}
