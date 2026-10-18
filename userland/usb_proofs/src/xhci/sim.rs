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

//! A hub whose every port flaps: connected for `period` ms, then empty for as
//! long, on the shim's clock. A reset enables a connected port at once. It
//! counts the resets each port was given and the devices addressed.

use std::cell::RefCell;

#[derive(Default)]
struct Hub {
    period: i64,
    enabled: [bool; 16],
    resets: [u32; 16],
    addressed: u32,
}

thread_local! {
    static HUB: RefCell<Hub> = RefCell::new(Hub::default());
}

/// A fresh hub whose ports flap every `period` ms.
pub fn flapping(period: i64) {
    HUB.with(|h| *h.borrow_mut() = Hub { period, ..Hub::default() });
}

fn connected() -> bool {
    let (now, period) = (nonos_libc::mk_uptime_ms(), HUB.with(|h| h.borrow().period));
    (now / period) % 2 == 0
}

pub(super) fn port_status(port: usize) -> [u8; 4] {
    let on = connected();
    let enabled = HUB.with(|h| {
        let mut h = h.borrow_mut();
        h.enabled[port] &= on;
        h.enabled[port]
    });
    // Connection, enable and power (USB 2 bit 8); the reset change bit
    // while the port is enabled.
    let status = u16::from(on) | u16::from(enabled) << 1 | 1 << 8;
    let [s0, s1] = status.to_le_bytes();
    [s0, s1, u8::from(enabled) << 4, 0]
}

pub(super) fn reset(port: usize) {
    let on = connected();
    HUB.with(|h| {
        let mut h = h.borrow_mut();
        h.resets[port] += 1;
        h.enabled[port] = on;
    });
}

pub(super) fn addressed() {
    HUB.with(|h| h.borrow_mut().addressed += 1);
}

/// The resets each port was given, from port 1.
pub fn resets() -> [u32; 16] {
    HUB.with(|h| h.borrow().resets)
}
