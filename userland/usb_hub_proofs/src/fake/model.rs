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
//! The state of a modelled hub: its ports, and what is plugged into each.

#[derive(Clone, Debug)]
pub enum Kind {
    /// A device the caller's classes bind.
    Bind,
    /// A device nothing binds.
    Ignore,
    /// Another hub.
    Hub(HubModel),
}

#[derive(Clone, Debug)]
pub struct Device {
    /// xHCI speed id the port reports once reset.
    pub speed: u8,
    pub kind: Kind,
}

#[derive(Clone, Debug, Default)]
pub struct PortModel {
    pub device: Option<Device>,
    pub enabled: bool,
    pub powered: bool,
    /// Status reads left before a reset ends; u8::MAX never ends.
    pub resetting: u8,
    pub c_connection: bool,
    pub c_reset: bool,
}

#[derive(Clone, Debug)]
pub struct HubModel {
    pub superspeed: bool,
    pub think: u8,
    pub ports: Vec<PortModel>,
}

impl HubModel {
    pub fn new(superspeed: bool, ports: usize) -> Self {
        Self { superspeed, think: 1, ports: vec![PortModel::default(); ports] }
    }

    /// `dev` plugged into `port` (from 1), its connection change set.
    pub fn plug(mut self, port: usize, dev: Device) -> Self {
        let p = &mut self.ports[port - 1];
        (p.device, p.c_connection) = (Some(dev), true);
        self
    }
}
