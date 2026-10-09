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
use nonos_libc::{mk_idle_ms, mk_uptime_ms, Deadline};

use super::settle::settle;
use crate::i2c_client::write_read;

// After RESET the device raises its interrupt and serves a zero-length report
// on the input register (HID over I2C v1.0 section 7.2.1). Linux waits up to
// one second for that interrupt and carries on without it (the
// I2C_HID_QUIRK_NO_IRQ_AFTER_RESET parts never send it); this driver reads
// the register instead of waiting on the line, every few milliseconds over
// the same second. A polled read cannot tell the sentinel from "nothing
// pending", so a sentinel seen early is held to the 100 ms Linux allows a
// part that resets without the interrupt.
pub(super) const RESET_TIMEOUT_MS: u64 = 1_000;
pub(super) const RESET_POLL_MS: u64 = 5;
pub(super) const RESET_MIN_MS: u64 = 100;

// Read the input register until the zero-length reset report appears, bounding
// the total wait in time so a missing device cannot hang the driver. Returns
// true when the reset sentinel was observed.
pub(super) fn await_reset(port: u32, addr: u8, input_reg: u16) -> bool {
    if input_reg == 0 {
        settle();
        return false;
    }
    let start = uptime();
    let deadline = Deadline::after_ms(RESET_TIMEOUT_MS);
    let reg = input_reg.to_le_bytes();
    loop {
        let mut drain = [0u8; 2];
        // Spec first: after reset the device auto-points at the input register
        // and the sentinel comes from a bare read. Devices that only answer a
        // register-addressed read get the fallback.
        let n = match write_read(port, addr, &[], &mut drain) {
            Some(n) if n >= 2 => Some(n),
            _ => write_read(port, addr, &reg, &mut drain),
        };
        // A zero-length report is encoded as a 0x0000 length prefix.
        if n.is_some_and(|n| n >= 2) && u16::from_le_bytes([drain[0], drain[1]]) == 0 {
            let spent = uptime().saturating_sub(start);
            if spent < RESET_MIN_MS {
                let _ = mk_idle_ms(RESET_MIN_MS - spent);
            }
            return true;
        }
        if deadline.expired() {
            return false;
        }
        let _ = mk_idle_ms(RESET_POLL_MS);
    }
}

fn uptime() -> u64 {
    mk_uptime_ms().max(0) as u64
}
