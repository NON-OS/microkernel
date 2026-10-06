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
//! waited for on the clock (USB 2.0 section 11.5.1.5, Linux
//! `hub_port_reset` and `hub_port_wait_reset`).

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use super::error::HubError;
use super::port_status::HubPort;
use super::read::port_status;
use super::request::{
    clear_port_feature, set_port_feature, C_PORT_CONNECTION, C_PORT_RESET, PORT_RESET,
};

/// Linux HUB_RESET_TIMEOUT: a reset still running after this has failed.
const RESET_TIMEOUT_MS: i64 = 800;
/// How often the port is read while it resets.
const RESET_POLL_MS: u64 = 10;
/// TRSTRCY, the time a device gets after reset before its first request
/// (USB 2.0 section 7.1.7.5).
const RESET_RECOVERY_MS: u64 = 10;

/// Reset `port` of the hub in `slot`; the port's status once enabled.
pub fn reset_port(
    xhci_port: u32,
    slot: u8,
    port: u8,
    superspeed: bool,
) -> Result<HubPort, HubError> {
    if !set_port_feature(xhci_port, slot, port, PORT_RESET) {
        return Err(HubError::ResetRefused);
    }
    let deadline = mk_uptime_ms().saturating_add(RESET_TIMEOUT_MS);
    let st = loop {
        let _ = mk_idle_ms(RESET_POLL_MS);
        let st = port_status(xhci_port, slot, port, superspeed).ok_or(HubError::PortStatus)?;
        if !st.connected {
            return Err(HubError::Gone);
        }
        if !st.resetting && (st.reset_changed || st.enabled) {
            break st;
        }
        if mk_uptime_ms() >= deadline {
            return Err(HubError::ResetTimeout);
        }
    };
    // The reset's own change bits are cleared, connection included, so the
    // next look at the port does not take the new device for a swapped one
    // (Linux `hub_port_finish_reset`).
    let _ = clear_port_feature(xhci_port, slot, port, C_PORT_RESET);
    let _ = clear_port_feature(xhci_port, slot, port, C_PORT_CONNECTION);
    if !st.enabled {
        return Err(HubError::NotEnabled);
    }
    let _ = mk_idle_ms(RESET_RECOVERY_MS);
    Ok(st)
}
