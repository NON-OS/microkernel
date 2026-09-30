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

//! The client side of driver.xhci0 this class driver uses: find the ports,
//! address the device, and move BOT traffic over its bulk pipes.

mod bulk;
mod call;
mod control;
mod lookup;
mod port;
mod slot;
mod wire;

pub use bulk::{bulk_in, bulk_out, configure_bulk, reset_bulk};
pub use control::{config_descriptor, control_no_data};
pub use lookup::lookup;
pub use port::{connected_ports, Port};
pub use slot::{address_device, disable_slot, enable_slot};
pub use wire::{BULK_MAX, E_BUSY, E_PIPE, PORT_CLAIMED, PORT_FREE};
