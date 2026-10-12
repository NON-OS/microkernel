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

use super::model::{Device, HubModel, PortModel};

#[derive(Default)]
pub struct Fake {
    pub hubs: HashMap<u8, HubModel>,
    /// What each addressed slot holds.
    pub devices: HashMap<u8, Device>,
    pub next_slot: u8,
    pub addressed: Vec<(u8, Place)>,
    pub disabled: Vec<u8>,
    pub configured: Vec<(u8, u8, u8)>,
    pub claimed: Vec<u8>,
    pub refuse_address: Option<i32>,
    pub refuse_configure: Option<i32>,
    pub last_reset: Option<(u8, usize)>,
    pub now: i64,
    pub lines: Vec<String>,
}

impl Fake {
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
