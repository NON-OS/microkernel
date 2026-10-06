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

//! USB descriptors as the network drivers read them. Pure: the proofs run
//! them over descriptors dumped from real devices.

mod device;
mod endpoint;
mod functional;
mod interface;
mod kinds;
mod mac;
mod walk;

pub use device::{config_header, device_info, DeviceInfo};
pub use functional::{find_functional, functional, union_data};
pub use interface::{interfaces, Interface};
pub use kinds::{CLASS_CDC_DATA, CLASS_COMM, CLASS_VENDOR, CONFIG, CS_INTERFACE, DEVICE};
pub use kinds::{ENDPOINT, INTERFACE, SS_ENDPOINT_COMPANION, STRING};
pub use mac::{mac_from_string, usable};
pub use walk::{walk, Walk};
