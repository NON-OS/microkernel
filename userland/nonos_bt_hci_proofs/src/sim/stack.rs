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

//! The host over the USB transport over the simulated controller, the
//! three endpoints a typical controller reports: events on 0x81 with a
//! 16-octet packet, ACL in on 0x82 and out on 0x02 with 64.

use super::config::Config;
use super::pipes::SimPipes;
use nonos_bt_hci::usb::{BtInterface, Endpoint, UsbHci};
use nonos_bt_hci::Host;

pub type Stack = Host<UsbHci<SimPipes>>;

pub const IFACE: BtInterface = BtInterface {
    number: 0,
    events: Endpoint { address: 0x81, max_packet: 16, interval: 1 },
    acl_in: Endpoint { address: 0x82, max_packet: 64, interval: 0 },
    acl_out: Endpoint { address: 0x02, max_packet: 64, interval: 0 },
};

pub fn stack(cfg: Config) -> Stack {
    Host::new(UsbHci::new(SimPipes::new(cfg), &IFACE, 27))
}

pub fn stack_with_budget(cfg: Config, budget: u32) -> Stack {
    Host::with_budget(UsbHci::new(SimPipes::new(cfg), &IFACE, 27), budget)
}

/// The opcodes the host sent, in order.
pub fn opcodes(s: &mut Stack) -> Vec<u16> {
    let cmds = &s.transport().pipes().commands;
    cmds.iter().map(|(_, c, _)| u16::from_le_bytes([c[0], c[1]])).collect()
}
