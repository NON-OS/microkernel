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
use nonos_libc::mk_idle_ms;

use super::action::{port_action, PortAction};
use super::debounce::debounce;
use super::power::power_on;
use super::wait_reset::wait_reset;
use crate::constants::{PORTSC_CCS, PORTSC_PR, PORTSC_WPR};
use crate::controller::park::IrqWait;
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{portsc_clear_changes, portsc_neutral, portsc_read, portsc_write};

/// Reset recovery (TRSTRCY, 10 ms) plus the margin Linux gives devices.
const RESET_RECOVERY_MS: u64 = 50;

/// Power, debounce and enable `port`. `usb3` is the port's protocol from the
/// Supported Protocol capability. Returns PORTSC once the port is enabled.
/// Between polls of PORTSC the waits block on `irq`, the controller's
/// interrupt, which a Port Status Change event raises.
pub fn reset_port(op_base: u64, port: u8, usb3: bool, irq: &mut IrqWait) -> XhciResult<u32> {
    let first = power_on(op_base, port, irq);
    if (first & PORTSC_CCS) == 0 {
        return Err(XhciError::NoDeviceOnPort);
    }
    let now = debounce(op_base, port)?;
    portsc_clear_changes(op_base, port, now);
    let bit = match port_action(portsc_read(op_base, port), usb3) {
        PortAction::Reset => PORTSC_PR,
        PortAction::WarmReset => PORTSC_WPR,
    };
    portsc_write(op_base, port, portsc_neutral(portsc_read(op_base, port)) | bit);
    let done = wait_reset(op_base, port, irq)?;
    portsc_clear_changes(op_base, port, done);
    let _ = mk_idle_ms(RESET_RECOVERY_MS);
    let after = portsc_read(op_base, port);
    if after & PORTSC_CCS == 0 {
        return Err(XhciError::NoDeviceOnPort);
    }
    Ok(after)
}
