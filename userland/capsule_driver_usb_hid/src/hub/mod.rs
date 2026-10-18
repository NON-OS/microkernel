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

//! The USB hub class (USB 2.0 chapter 11, USB 3.2 chapter 10, Linux
//! drivers/usb/core/hub.c): a hub's descriptor and ports, bringing up the
//! device on a port, and where a device below a hub sits.

mod attach;
mod bringup;
mod budget;
mod descriptor;
mod error;
mod node;
mod place;
mod port_status;
mod power;
mod read;
mod request;
mod route;
mod say;
mod stage;
mod status_endpoint;
mod step;
mod timing;

pub use attach::attach;
pub use budget::Budget;
pub use error::HubError;
pub use node::Hub;
pub use place::{speed_name, Place};
pub use port_status::{HubPort, SPEED_HIGH};
pub use read::port_status;
pub use request::{
    clear_port_feature, set_port_feature, C_PORT_CONNECTION, C_PORT_RESET, PORT_RESET,
};
pub use route::{child_route, child_tt};
pub use say::{hub_port, Line};
pub use step::Step;
