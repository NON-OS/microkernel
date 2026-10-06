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

//! The client side of driver.xhci0: find the ports, address a device, and
//! move its traffic over endpoint 0 and its bulk pipes.

mod bulk;
mod call;
mod control;
mod lookup;
mod port;
mod slot;
mod wire;
mod xhci_bus;

pub use lookup::lookup;
pub use port::connected_ports;
pub use slot::{address_device, disable_slot, enable_slot};
pub use wire::{BULK_MAX, CONTROL_MAX, E_AGAIN, E_BUSY, E_INVAL, E_IO, E_PIPE};
pub use wire::{PORT_CLAIMED, PORT_FREE};
pub use xhci_bus::XhciBus;
