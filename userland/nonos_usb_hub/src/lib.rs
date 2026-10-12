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
//! The USB hub class (USB 2.0 chapter 11, USB 3.2 chapter 10): a hub's
//! descriptor and ports, resetting a port, where a device below a hub
//! sits, and the walk that brings up and forgets the devices on a hub's
//! ports. The controller, the clock and the log are reached through
//! `Host`; what a device is bound to is the caller's, through `Classes`.

#![no_std]

extern crate alloc;

mod attach;
mod attach_child;
mod classes;
mod descriptor;
mod error;
mod forget_slot;
mod host;
mod line;
mod node;
mod place;
mod port_status;
mod power;
mod read;
mod request;
mod reset;
mod route;
mod scan_hub;
mod scan_hubs;
mod status_endpoint;

pub use attach::attach;
pub use classes::Classes;
pub use descriptor::{parse_hub_descriptor, HubDescriptor, DT_HUB, DT_SS_HUB, MAX_HUB_PORTS};
pub use error::HubError;
pub use forget_slot::{forget_root, forget_slot};
pub use host::{Host, Setup, E_BUSY, E_INVAL};
pub use line::{hub_port, Line};
pub use node::Hub;
pub use place::{speed_name, Place};
pub use port_status::{
    decode_port_status, HubPort, SPEED_FULL, SPEED_HIGH, SPEED_LOW, SPEED_SUPER,
};
pub use read::port_status;
pub use request::{clear_port_feature, C_PORT_CONNECTION};
pub use reset::reset_port;
pub use route::{child_route, child_tt, Tt, MAX_HUB_DEPTH};
pub use scan_hubs::scan_hubs;
pub use status_endpoint::{status_endpoint, StatusEndpoint};
