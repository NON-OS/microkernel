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

//! One step of bringing up the device on a hub port, as `Bringup` says,
//! with what the step asks of the hub and the host. While another port of
//! the host is reset or recovering, this one waits its turn: a device fresh
//! off its reset answers at address 0, and only one may (USB 2.0 9.1.1.4,
//! as recalled, unverified).

use alloc::vec::Vec;

use crate::attach_child::attach_child;
use crate::classes::Classes;
use crate::error::HubError;
use crate::host::Host;
use crate::line::hub_port;
use crate::node::Hub;
use crate::place::speed_name;
use crate::port_status::HubPort;
use crate::request::{
    clear_port_feature, set_port_feature, C_PORT_CONNECTION, C_PORT_RESET, PORT_RESET,
};
use crate::step::Step;

pub(crate) fn step_port<H: Host>(
    h: &mut H,
    hub: &mut Hub,
    port: u8,
    st: HubPort,
    resetting: &mut bool,
    hubs: &mut Vec<Hub>,
    classes: &mut impl Classes<H>,
) {
    let i = port as usize - 1;
    let now = h.now_ms().max(0) as u64;
    let may_reset = !*resetting || hub.bring[i].resetting();
    let step = hub.bring[i].step(st, now, may_reset);
    *resetting |= hub.bring[i].resetting();
    let failed = match step {
        Step::Wait => None,
        Step::Reset if set_port_feature(h, hub.slot, port, PORT_RESET) => None,
        Step::Reset => {
            hub.bring[i].end_try();
            Some(HubError::ResetRefused)
        }
        Step::ResetOver(failed) => {
            let _ = clear_port_feature(h, hub.slot, port, C_PORT_RESET);
            let _ = clear_port_feature(h, hub.slot, port, C_PORT_CONNECTION);
            failed
        }
        Step::Address(speed) => match attach_child(h, hub, port, speed, hubs, classes) {
            Ok(kept) => {
                hub.bring[i].decide();
                hub.child[i] = kept.unwrap_or(0);
                let what: &[u8] = if kept.is_some() {
                    b" device bound"
                } else {
                    b" device, nothing bound; left alone"
                };
                hub_port(hub.slot, port).text(speed_name(speed)).text(what).say(h);
                None
            }
            Err(e) => Some(e),
        },
        Step::Failed(e) => Some(e),
    };
    if let Some(e) = failed {
        hub_port(hub.slot, port).text(b"stopped: ").text(e.says()).say(h);
    }
}
