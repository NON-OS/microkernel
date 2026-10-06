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

//! A hub this driver configured, and what it keeps for each of its ports.

use super::descriptor::MAX_HUB_PORTS;
use super::place::Place;

const PORTS: usize = MAX_HUB_PORTS as usize;

pub struct Hub {
    pub slot: u8,
    pub place: Place,
    pub ports: u8,
    /// Per port, from port 1: the slot kept for the device there, 0 for
    /// none. A device that bound nothing has its slot given back.
    pub child: [u8; PORTS],
    /// Per port: the tries that came to nothing since it was last empty.
    pub tries: [u8; PORTS],
}

impl Hub {
    pub fn new(slot: u8, place: Place, ports: u8) -> Self {
        Self { slot, place, ports: ports.min(MAX_HUB_PORTS), child: [0; PORTS], tries: [0; PORTS] }
    }
}
