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

//! Where an addressed device sits: its root port, its route below it, how
//! many hubs are above it, its speed, and the TT it is reached through.

use super::port_status::SPEED_SUPER;
use super::route::Tt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Place {
    pub root_port: u8,
    pub route: u32,
    /// Hubs between the root port and the device; 0 on a root port.
    pub depth: u8,
    pub speed: u8,
    pub tt: Option<Tt>,
}

impl Place {
    /// A device straight on root port `port`.
    pub fn root(port: u8, speed: u8) -> Self {
        Self { root_port: port, route: 0, depth: 0, speed, tt: None }
    }

    pub fn superspeed(&self) -> bool {
        self.speed >= SPEED_SUPER
    }
}

pub fn speed_name(speed: u8) -> &'static [u8] {
    match speed {
        1 => b"full speed",
        2 => b"low speed",
        3 => b"high speed",
        4 => b"SuperSpeed",
        _ => b"SuperSpeed Plus",
    }
}
