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

//! The root ports driver.xhci0 sees a device on.

use alloc::vec::Vec;

use super::call::call;
use super::port_list::{connected, Port};
use super::wire::{DATA_AT, OP_PORT_STATUS};

/// Every root port with a device connected.
pub fn connected_ports(xhci: u32) -> Result<Vec<Port>, i32> {
    let mut resp = [0u8; DATA_AT + 4 + 255 * 8];
    let len = call(xhci, OP_PORT_STATUS, &[], &mut resp)?;
    connected(&resp, len)
}
