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
use nonos_libc::{mk_idle_ms, Deadline};

use crate::constants::PORTSC_CCS;
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{controller_failed, portsc_read};

/// Connection debounce (USB 2.0 TATTDB, Linux HUB_DEBOUNCE_*): the connect
/// status must hold for 100 ms, sampled every 25 ms, within 1.5 s.
const DEBOUNCE_STABLE_MS: u64 = 100;
const DEBOUNCE_STEP_MS: u64 = 25;
const DEBOUNCE_TIMEOUT_MS: u64 = 1_500;

/// Wait for the connect status to hold for `DEBOUNCE_STABLE_MS`. A port
/// that keeps bouncing past the timeout, or that ends disconnected, has no
/// device to address. A failed controller ends the wait at once: one gone
/// from the bus reads all ones, a connect that holds.
pub(super) fn debounce(op_base: u64, port: u8) -> XhciResult<u32> {
    let deadline = Deadline::after_ms(DEBOUNCE_TIMEOUT_MS);
    let mut stable_ms = 0u64;
    let mut last = portsc_read(op_base, port);
    loop {
        if controller_failed(op_base) {
            return Err(XhciError::ControllerDead);
        }
        if deadline.expired() {
            return Err(XhciError::NoDeviceOnPort);
        }
        let _ = mk_idle_ms(DEBOUNCE_STEP_MS);
        let now = portsc_read(op_base, port);
        if (now ^ last) & PORTSC_CCS == 0 {
            stable_ms += DEBOUNCE_STEP_MS;
        } else {
            stable_ms = 0;
        }
        last = now;
        if stable_ms >= DEBOUNCE_STABLE_MS {
            if now & PORTSC_CCS == 0 {
                return Err(XhciError::NoDeviceOnPort);
            }
            return Ok(now);
        }
    }
}
