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
#[path = "../../../nonos_usbnet/src/hosts.rs"]
mod hosts;
#[path = "../../../nonos_usbnet/src/hosts_first.rs"]
mod hosts_first;
mod lookup;
mod pair;
mod port;
mod slot;
#[path = "../../../../src/services/registry/usb_hosts.rs"]
mod usb_hosts;
mod wire;

pub use bulk::{bulk_in_on, bulk_out_on, reset_bulk_on};
pub use control::{config_descriptor, control_in, control_no_data};
pub use hosts::Hosts;
pub use lookup::{host, hosts};
pub use pair::DATA_PAIR;
pub use pair::{bulk_in, bulk_out, configure_bulk, configure_bulk2, configure_streams, reset_bulk};
pub use port::{connected_ports, port_speed, Port};
pub use slot::{address_device, disable_slot, enable_slot};
pub use wire::{BULK_MAX, E_BUSY, E_PIPE, PORT_CLAIMED, PORT_FREE};
