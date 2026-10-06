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

//! What the driver holds: the HID endpoints it reads and the hubs it
//! configured.

use alloc::vec::Vec;

use crate::hub::Hub;

use super::types::HidEndpoint;

pub struct Devices {
    pub eps: Vec<HidEndpoint>,
    pub hubs: Vec<Hub>,
}

impl Devices {
    pub fn new() -> Self {
        Self { eps: Vec::new(), hubs: Vec::new() }
    }

    /// Whether `slot` is kept: an endpoint is bound on it or it is a hub.
    pub fn holds(&self, slot: u8) -> bool {
        self.eps.iter().any(|e| e.slot == slot) || self.hubs.iter().any(|h| h.slot == slot)
    }
}
