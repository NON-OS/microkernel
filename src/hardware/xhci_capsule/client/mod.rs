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

mod address_device;
mod config_descriptor;
mod controller_status;
mod device_descriptor;
mod disable_slot;
mod enable_slot;
mod healthcheck;
mod port_status;
mod seq;
mod transport;

pub(super) use transport::REPLY_INBOX;

pub use address_device::{address_device, AddressedDevice};
pub use config_descriptor::{config_descriptor, CONFIG_DESCRIPTOR_MAX_LEN};
pub use controller_status::{controller_status, ControllerStatus};
pub use device_descriptor::{device_descriptor, DEVICE_DESCRIPTOR_LEN};
pub use disable_slot::disable_slot;
pub use enable_slot::enable_slot;
pub use healthcheck::healthcheck;
pub use port_status::{port_status, PortSnapshot};
