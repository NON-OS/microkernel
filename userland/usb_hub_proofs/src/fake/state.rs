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

//! What the fake host holds: the modelled hubs, the slots it gave, and
//! every request, refusal and log line, for the tests to read.

use std::collections::HashMap;

use nonos_usb_hub::Place;

use super::model::{Device, HubModel, Kind, PortModel};

#[derive(Default)]
pub struct Fake {
    pub hubs: HashMap<u8, HubModel>,
    /// What each addressed slot holds.
    pub devices: HashMap<u8, Device>,
    pub next_slot: u8,
    pub addressed: Vec<(u8, Place)>,
    pub disabled: Vec<u8>,
    pub parked: Vec<(u8, Place, u16)>,
    pub configured: Vec<(u8, u8, u8)>,
    pub claimed: Vec<u8>,
    pub refuse_address: Option<i32>,
    pub refuse_configure: Option<i32>,
    pub last_reset: Option<(u8, usize)>,
    /// Every port reset asked for; how long the walk slept.
    pub resets: Vec<(u8, usize)>,
    pub slept: u64,
    pub now: i64,
    pub lines: Vec<String>,
}

impl Fake {
    /// The device on the port reset last, addressed in `slot` at `place`;
    /// a hub's model goes with it.
    pub fn addressed_at(&mut self, slot: u8, place: &Place) {
        let (hub, port) = self.last_reset.expect("a port was reset first");
        let dev = self.port(hub, port).device.clone().expect("a device on the reset port");
        if let Kind::Hub(model) = &dev.kind {
            self.hubs.insert(slot, model.clone());
        }
        self.devices.insert(slot, dev);
        self.addressed.push((slot, *place));
    }

    pub fn port(&mut self, hub_slot: u8, port: usize) -> &mut PortModel {
        &mut self.hubs.get_mut(&hub_slot).expect("hub").ports[port - 1]
    }

    /// The device on `port` leaves.
    pub fn unplug(&mut self, hub_slot: u8, port: usize) {
        let p = self.port(hub_slot, port);
        (p.device, p.enabled, p.c_connection) = (None, false, true);
    }

    pub fn said(&self, words: &str) -> bool {
        self.lines.iter().any(|l| l.contains(words))
    }
}
