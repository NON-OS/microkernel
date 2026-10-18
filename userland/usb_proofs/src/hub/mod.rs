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

//! The hub class, loaded from the driver's own files. Its requests go to the
//! controller driver played in `crate::xhci`, so bringing up a hub's ports
//! runs here as it does in the driver.

#[path = "../../../capsule_driver_usb_hid/src/hub/bringup.rs"]
pub mod bringup;
#[path = "../../../capsule_driver_usb_hid/src/hub/budget.rs"]
pub mod budget;
#[path = "../../../capsule_driver_usb_hid/src/hub/descriptor.rs"]
pub mod descriptor;
#[path = "../../../capsule_driver_usb_hid/src/hub/error.rs"]
pub mod error;
#[path = "../../../capsule_driver_usb_hid/src/hub/node.rs"]
pub mod node;
#[path = "../../../capsule_driver_usb_hid/src/hub/place.rs"]
pub mod place;
#[path = "../../../capsule_driver_usb_hid/src/hub/port_status.rs"]
pub mod port_status;
#[path = "../../../capsule_driver_usb_hid/src/hub/read.rs"]
pub mod read;
#[path = "../../../capsule_driver_usb_hid/src/hub/request.rs"]
pub mod request;
#[path = "../../../capsule_driver_usb_hid/src/hub/route.rs"]
pub mod route;
#[path = "../../../capsule_driver_usb_hid/src/hub/say.rs"]
pub mod say;
#[path = "../../../capsule_driver_usb_hid/src/hub/stage.rs"]
pub mod stage;
#[path = "../../../capsule_driver_usb_hid/src/hub/status_endpoint.rs"]
pub mod status_endpoint;
#[path = "../../../capsule_driver_usb_hid/src/hub/step.rs"]
pub mod step;
#[path = "../../../capsule_driver_usb_hid/src/hub/timing.rs"]
pub mod timing;

pub use error::HubError;
pub use node::Hub;
pub use place::{speed_name, Place};
pub use port_status::{HubPort, SPEED_HIGH};
pub use read::port_status;
pub use request::{
    clear_port_feature, set_port_feature, C_PORT_CONNECTION, C_PORT_RESET, PORT_RESET,
};
pub use route::{child_route, child_tt};
pub use say::hub_port;
pub use step::Step;

#[cfg(test)]
mod bringup_failure_tests;
#[cfg(test)]
mod bringup_steps;
#[cfg(test)]
mod bringup_swap_tests;
#[cfg(test)]
mod bringup_tests;
