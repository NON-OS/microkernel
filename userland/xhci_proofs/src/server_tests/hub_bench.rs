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

//! Requests a hub class driver sends, as `capsule_driver_usb_hid` lays
//! them out (`xhci/ops/address_routed.rs`, `configure_hub.rs`), and what
//! the controller was asked to do with them.

use super::bench::Bench;
use super::hc_input::Ran;
use super::wire::Reply;
use crate::protocol::{OP_ADDRESS_DEVICE, OP_ADDRESS_ROUTED, OP_CONFIGURE_HUB};

pub const FULL: u8 = 1;
pub const LOW: u8 = 2;
pub const HIGH: u8 = 3;
pub const SUPER: u8 = 4;
pub const ADDRESS_DEVICE: u32 = 11;
/// `slot, root_port, speed, route, tt_slot, tt_port`.
pub type Routed = (u8, u8, u8, u32, u8, u8);

pub fn routed_body((slot, port, speed, route, tt_slot, tt_port): Routed) -> Vec<u8> {
    let r = route.to_le_bytes();
    vec![slot, port, speed, r[0], r[1], r[2], r[3], tt_slot, tt_port]
}
pub const CONFIGURE_ENDPOINT: u32 = 12;

impl Bench {
    /// A slot addressed on root port 1, the device the port model shows.
    pub fn root_device(&mut self) -> u8 {
        let slot = self.enable_slot();
        assert_eq!(self.call(OP_ADDRESS_DEVICE, &[slot, 1]).status, 0, "Address Device");
        slot
    }

    pub fn configure_hub(&mut self, slot: u8, ports: u8, think_time: u8, mtt: u8) -> i32 {
        self.call(OP_CONFIGURE_HUB, &[slot, ports, think_time, mtt]).status
    }

    /// OP_ADDRESS_ROUTED for `slot` on root port 1.
    pub fn routed(&mut self, slot: u8, speed: u8, route: u32, tt: (u8, u8)) -> Reply {
        self.call(OP_ADDRESS_ROUTED, &routed_body((slot, 1, speed, route, tt.0, tt.1)))
    }

    /// A hub of `ports` ports at `route` below root port 1, configured.
    pub fn hub_below(&mut self, speed: u8, route: u32, tt: (u8, u8), ports: u8) -> u8 {
        let slot = self.enable_slot();
        assert_eq!(self.routed(slot, speed, route, tt).status, 0, "hub at {route:#x} addressed");
        assert_eq!(self.configure_hub(slot, ports, 0, 0), 0, "hub at {route:#x} configured");
        slot
    }

    /// The last command of TRB type `ty` the controller ran.
    pub fn last(&self, ty: u32) -> Ran {
        let ran = self.ran();
        *ran.iter().rev().find(|r| r.trb.get_type() == ty).expect("the command ran")
    }
}
