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

//! The driver's port bookkeeping, loaded from its own files. Hubs are
//! driver.usb_hub0's; their bring-up is proven in usb_hub_proofs.

#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/devices.rs"]
pub mod devices;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/drop_port.rs"]
pub mod drop_port;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/types.rs"]
pub mod types;

pub use devices::Devices;
pub use types::HidEndpoint;

#[cfg(test)]
mod unplug_pointer_tests;
#[cfg(test)]
mod unplug_release_tests;
#[cfg(test)]
mod unplug_slot_tests;
