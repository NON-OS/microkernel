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
//! A key found brought up: its interrupt IN and OUT endpoints given rings,
//! and a channel of our own asked for on the broadcast one.

use nonos_libc::{mk_idle_ms, mk_uptime_ms, Deadline};
use nonos_usbnet::xhci::XhciBus;
use nonos_usbnet::Bus;

use crate::ctaphid::{allocate, Channel, Key, CAP_CBOR};

/// How long the key has to answer INIT.
const INIT_MS: u64 = 1_000;
const E_NOTSUP: i32 = -95;

pub type Up = ((u8, u8), Channel);

pub fn up(bus: &mut XhciBus, key: &Key) -> Result<Up, (&'static str, i32)> {
    let input = bus.configure_interrupt(key.input).map_err(|e| ("IN endpoint", e))?;
    let output = bus.configure_interrupt(key.output).map_err(|e| ("OUT endpoint", e))?;
    let ch = channel(bus, (input, output)).map_err(|e| ("channel", e))?;
    // A CTAP1-only key takes no CBOR, the one thing passed through here.
    if ch.caps & CAP_CBOR == 0 {
        return Err(("a CTAP1-only key", E_NOTSUP));
    }
    Ok(((input, output), ch))
}

/// A channel asked for on endpoints `dci`: at bind, and again when the
/// last one is spent.
pub fn channel(bus: &mut XhciBus, dci: (u8, u8)) -> Result<Channel, i32> {
    let deadline = Deadline::after_ms(INIT_MS);
    let wait = || {
        let _ = mk_idle_ms(1);
        !deadline.expired()
    };
    allocate(bus, dci, nonce(), wait)
}

/// Uptime and a count: unique for this driver, the key's one host.
fn nonce() -> [u8; 8] {
    static COUNT: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
    let n = COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    ((mk_uptime_ms() as u64) << 16 ^ n as u64).to_le_bytes()
}
