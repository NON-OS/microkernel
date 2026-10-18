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

//! One step of bringing up the device on a hub port, as `hub::Bringup`
//! says, with what the step asks of the hub and the controller driver.

use nonos_libc::mk_uptime_ms;

use crate::hub::{clear_port_feature, hub_port, set_port_feature, speed_name, Hub, HubPort, Step};
use crate::hub::{HubError, C_PORT_CONNECTION, C_PORT_RESET, PORT_RESET};

use super::attach_child::attach_child;
use super::devices::Devices;

pub(super) fn step_port(xhci_port: u32, hub: &mut Hub, port: u8, st: HubPort, devs: &mut Devices) {
    let i = port as usize - 1;
    let now = mk_uptime_ms().max(0) as u64;
    let failed = match hub.bring[i].step(st, now) {
        Step::Wait => None,
        Step::Reset if set_port_feature(xhci_port, hub.slot, port, PORT_RESET) => None,
        Step::Reset => {
            hub.bring[i].end_try();
            Some(HubError::ResetRefused)
        }
        Step::ResetOver(failed) => {
            let _ = clear_port_feature(xhci_port, hub.slot, port, C_PORT_RESET);
            let _ = clear_port_feature(xhci_port, hub.slot, port, C_PORT_CONNECTION);
            failed
        }
        Step::Address(speed) => match attach_child(xhci_port, hub, port, speed, devs) {
            Ok(kept) => {
                hub.bring[i].decide();
                hub.child[i] = kept.unwrap_or(0);
                let what: &[u8] = match kept {
                    Some(_) => b" device bound",
                    None => b" device, no keyboard, mouse or hub this driver reads; left alone",
                };
                hub_port(hub.slot, port).text(speed_name(speed)).text(what).say();
                None
            }
            Err(e) => Some(e),
        },
        Step::Failed(e) => Some(e),
    };
    if let Some(e) = failed {
        hub_port(hub.slot, port).text(b"stopped: ").text(e.says()).say();
    }
}
