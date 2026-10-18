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

//! The driver's port bookkeeping and hub port scan, loaded from its own
//! files; only the tests call the scan, so it is built only for them.
//! Classifying a device takes descriptor reads this harness does not play,
//! so `classify` here finds nothing to bind: the slot is given back.

#[cfg(test)]
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/attach_child.rs"]
mod attach_child;
#[cfg(test)]
mod classify;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/devices.rs"]
pub mod devices;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/drop_port.rs"]
pub mod drop_port;
#[cfg(test)]
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/forget_slot.rs"]
mod forget_slot;
#[cfg(test)]
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/scan_hub.rs"]
mod scan_hub;
#[cfg(test)]
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/step_port.rs"]
mod step_port;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/enumerate/types.rs"]
pub mod types;

pub use devices::Devices;
pub use types::HidEndpoint;

#[cfg(test)]
mod hub_flap_tests;
#[cfg(test)]
mod hub_power_tests;
#[cfg(test)]
mod hub_stream_tests;
#[cfg(test)]
mod unplug_pointer_tests;
#[cfg(test)]
mod unplug_release_tests;
#[cfg(test)]
mod unplug_slot_tests;
