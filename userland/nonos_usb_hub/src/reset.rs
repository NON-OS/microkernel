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
//! Resetting one hub port so the device behind it answers at address 0,
//! waited for on the clock (USB 2.0 section 11.5.1.5).

use crate::error::HubError;
use crate::host::Host;
use crate::port_status::HubPort;
use crate::read::port_status;
use crate::request::{
    clear_port_feature, set_port_feature, C_PORT_CONNECTION, C_PORT_RESET, PORT_RESET,
};

/// A reset still running after this has failed (Linux's hub driver waits
/// as long; observed, unverified).
const RESET_TIMEOUT_MS: i64 = 800;
/// How often the port is read while it resets.
const RESET_POLL_MS: u64 = 10;
/// TRSTRCY, the time a device gets after reset before its first request
/// (USB 2.0 section 7.1.7.5).
const RESET_RECOVERY_MS: u64 = 10;

/// Reset `port` of the hub in `slot`; the port's status once enabled.
pub fn reset_port(
    h: &mut impl Host,
    slot: u8,
    port: u8,
    superspeed: bool,
) -> Result<HubPort, HubError> {
    if !set_port_feature(h, slot, port, PORT_RESET) {
        return Err(HubError::ResetRefused);
    }
    let deadline = h.now_ms().saturating_add(RESET_TIMEOUT_MS);
    let st = loop {
        h.idle_ms(RESET_POLL_MS);
        let st = port_status(h, slot, port, superspeed).ok_or(HubError::PortStatus)?;
        if !st.connected {
            return Err(HubError::Gone);
        }
        if !st.resetting && (st.reset_changed || st.enabled) {
            break st;
        }
        if h.now_ms() >= deadline {
            return Err(HubError::ResetTimeout);
        }
    };
    // The reset's own change bits are cleared, connection included, so the
    // next look at the port does not take the new device for a swapped one.
    let _ = clear_port_feature(h, slot, port, C_PORT_RESET);
    let _ = clear_port_feature(h, slot, port, C_PORT_CONNECTION);
    if !st.enabled {
        return Err(HubError::NotEnabled);
    }
    h.idle_ms(RESET_RECOVERY_MS);
    Ok(st)
}
