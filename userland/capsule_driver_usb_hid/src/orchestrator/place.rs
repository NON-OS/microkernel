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

//! Where a bound device sits, as this driver sees it: the port it is on,
//! a root port or a virtual port driver.usb_hub0 gives for a device below a
//! hub, and its speed. Hubs themselves are driver.usb_hub0's.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Place {
    pub root_port: u8,
    pub speed: u8,
}

impl Place {
    pub fn root(port: u8, speed: u8) -> Self {
        Self { root_port: port, speed }
    }
}
