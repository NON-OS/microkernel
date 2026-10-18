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

//! What the driver holds: the HID endpoints it reads.

use alloc::vec::Vec;

use super::types::HidEndpoint;

#[derive(Default)]
pub struct Devices {
    pub eps: Vec<HidEndpoint>,
}

impl Devices {
    pub fn new() -> Self {
        Self { eps: Vec::new() }
    }

    /// Whether `slot` is kept: an endpoint is bound on it.
    pub fn holds(&self, slot: u8) -> bool {
        self.eps.iter().any(|e| e.slot == slot)
    }
}
