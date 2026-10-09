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

//! The hub class's pure parts, loaded from the driver's own files.

#[path = "../../../capsule_driver_usb_hid/src/hub/descriptor.rs"]
pub mod descriptor;
#[path = "../../../capsule_driver_usb_hid/src/hub/port_status.rs"]
pub mod port_status;
#[path = "../../../capsule_driver_usb_hid/src/hub/route.rs"]
pub mod route;
#[path = "../../../capsule_driver_usb_hid/src/hub/status_endpoint.rs"]
pub mod status_endpoint;
