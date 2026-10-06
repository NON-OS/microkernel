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
//! Bringing one root port to Enabled before Address Device (xHCI 1.2
//! sections 4.3 and 4.19, USB 2.0 section 7.1.7.3, Linux `hub_port_reset`).
//!
//! A USB 2 port is enabled only by a port reset. A USB 3 port enables itself
//! once its link trains, so one that already shows PED is used as it is; one
//! whose link fell to Inactive or Compliance comes back only through a warm
//! reset, and any other gets a hot reset. Every write is the neutral PORTSC
//! value with the one action bit added: writing back a PED of one would
//! disable the very port being brought up.

use nonos_libc::{mk_idle_ms, Deadline};

use crate::constants::{
    PLS_COMPLIANCE, PLS_INACTIVE, PORTSC_CCS, PORTSC_PED, PORTSC_PLS_MASK, PORTSC_PLS_SHIFT,
    PORTSC_PP, PORTSC_PR, PORTSC_PRC, PORTSC_WPR,
};
use crate::controller::park::{park_step, SPIN_BUDGET};
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{portsc_clear_changes, portsc_neutral, portsc_read, portsc_write};

const RESET_TIMEOUT_MS: u64 = 1_000;
/// Root hub power-on to power-good: xHCI root hubs report 20 ms.
pub const POWER_GOOD_MS: u64 = 20;
/// Connection debounce (USB 2.0 TATTDB, Linux HUB_DEBOUNCE_*): the connect
/// status must hold for 100 ms, sampled every 25 ms, within 1.5 s.
const DEBOUNCE_STABLE_MS: u64 = 100;
const DEBOUNCE_STEP_MS: u64 = 25;
const DEBOUNCE_TIMEOUT_MS: u64 = 1_500;
/// Reset recovery (TRSTRCY, 10 ms) plus the margin Linux gives devices.
const RESET_RECOVERY_MS: u64 = 50;

/// How a port is brought to Enabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortAction {
    /// Hot reset through PR.
    Reset,
    /// Warm reset through WPR: a USB 3 link in Inactive or Compliance.
    WarmReset,
}

/// What `portsc` calls for on a USB 3 port (`usb3`) or any other.
pub fn port_action(portsc: u32, usb3: bool) -> PortAction {
    if !usb3 {
        return PortAction::Reset;
    }
    let pls = (portsc & PORTSC_PLS_MASK) >> PORTSC_PLS_SHIFT;
    if pls == PLS_INACTIVE || pls == PLS_COMPLIANCE {
        return PortAction::WarmReset;
    }
    // A USB 3 port whose link trained is enabled already, but the device on it
    // may have been addressed by another class driver that probed it and let
    // it go. Real devices refuse a second Address Device without a reset in
    // between (QEMU's do not), so every port is reset before it is addressed,
    // as Linux hub_port_init does.
    PortAction::Reset
}

/// Power, debounce and enable `port`. `usb3` is the port's protocol from the
/// Supported Protocol capability. Returns PORTSC once the port is enabled.
pub fn reset_port(op_base: u64, port: u8, usb3: bool) -> XhciResult<u32> {
    let first = power_on(op_base, port);
    if (first & PORTSC_CCS) == 0 {
        return Err(XhciError::NoDeviceOnPort);
    }
    let now = debounce(op_base, port)?;
    portsc_clear_changes(op_base, port, now);
    let action = port_action(portsc_read(op_base, port), usb3);
    let bit = match action {
        PortAction::Reset => PORTSC_PR,
        PortAction::WarmReset => PORTSC_WPR,
    };
    portsc_write(op_base, port, portsc_neutral(portsc_read(op_base, port)) | bit);
    let done = wait_reset(op_base, port)?;
    portsc_clear_changes(op_base, port, done);
    let _ = mk_idle_ms(RESET_RECOVERY_MS);
    let after = portsc_read(op_base, port);
    if after & PORTSC_CCS == 0 {
        return Err(XhciError::NoDeviceOnPort);
    }
    Ok(after)
}

/// Wait for the reset to finish with the port enabled. PRC with PED clear
/// is a reset that ended with the device gone or not answering; on silicon
/// that state holds, so it is reported once the deadline passes.
fn wait_reset(op_base: u64, port: u8) -> XhciResult<u32> {
    let deadline = Deadline::after_ms(RESET_TIMEOUT_MS);
    let mut spins = 0u32;
    loop {
        let now = portsc_read(op_base, port);
        if (now & PORTSC_PRC) != 0 && (now & PORTSC_PED) != 0 {
            return Ok(now);
        }
        spins = spins.saturating_add(1);
        if spins > SPIN_BUDGET && deadline.expired() {
            return Err(XhciError::PortResetTimeout);
        }
        park_step(spins);
    }
}

/// Wait for the connect status to hold for `DEBOUNCE_STABLE_MS`. A port
/// that keeps bouncing past the timeout, or that ends disconnected, has no
/// device to address.
fn debounce(op_base: u64, port: u8) -> XhciResult<u32> {
    let deadline = Deadline::after_ms(DEBOUNCE_TIMEOUT_MS);
    let mut stable_ms = 0u64;
    let mut last = portsc_read(op_base, port);
    loop {
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

/// Port power on, when the controller switches it (HCCPARAMS1.PPC) and it
/// is off, then wait for a connect up to the power-good time.
fn power_on(op_base: u64, port: u8) -> u32 {
    let current = portsc_read(op_base, port);
    if (current & PORTSC_PP) == 0 {
        portsc_write(op_base, port, portsc_neutral(current) | PORTSC_PP);
    }
    let mut last = portsc_read(op_base, port);
    let deadline = Deadline::after_ms(POWER_GOOD_MS);
    let mut spins = SPIN_BUDGET;
    while (last & PORTSC_CCS) == 0 && !deadline.expired() {
        spins = spins.saturating_add(1);
        park_step(spins);
        last = portsc_read(op_base, port);
    }
    last
}

/// Power every root port the controller left off, and give them the
/// power-good time, so a device attached at boot shows its connect before
/// any class driver looks. A controller without port power switching
/// (HCCPARAMS1.PPC clear) keeps PP at one and this writes nothing.
pub fn power_all_ports(op_base: u64, max_ports: u8) {
    let mut wrote = false;
    for port in 1..=max_ports {
        let current = portsc_read(op_base, port);
        if current & PORTSC_PP == 0 {
            portsc_write(op_base, port, portsc_neutral(current) | PORTSC_PP);
            wrote = true;
        }
    }
    if wrote {
        let _ = mk_idle_ms(POWER_GOOD_MS);
    }
}
