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
//! The devices below hubs as a host lists them. driver.usb_hub0 walks
//! the hubs and parks each child it addresses (OP_PARK); the host lists
//! the child beside its root ports in PORT_STATUS as a virtual port, and
//! a class driver claims it as it claims a root port: ENABLE_SLOT, then
//! ADDRESS_DEVICE naming the virtual port. Its slot then stands for the
//! parked one (an alias), first claim wins, and a detach on the hub port
//! reaches it as a root-port detach does. Pure: a host keeps one `Virt`.

#![no_std]

mod claim;
mod disable;
mod park;
mod report;
mod table;

pub use disable::Disable;
pub use park::{parse_park, PARK_LEN};
pub use report::{ext, port_entry};
pub use table::{is_virtual, Parked, Virt, VPORTS, VPORT_BASE};

/// The op driver.usb_hub0 parks a child with (bodies in park.rs).
pub const OP_PARK: u16 = 0x0022;
