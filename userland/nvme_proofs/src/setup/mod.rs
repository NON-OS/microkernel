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

//! The real controller take and stop, enable, disable and teardown, over
//! the libc shim: the claim, BAR0 map, MSI-X bind and Command writes, and
//! the order bring-up makes them in up to Bus Master.

#[path = "../../../capsule_driver_nvme/src/setup/sequence/bring_up.rs"]
pub mod bring_up;
#[cfg(test)]
mod bus_master_tests;
#[path = "../../../capsule_driver_nvme/src/setup/claim.rs"]
pub mod claim;
#[path = "../../../capsule_driver_nvme/src/setup/device.rs"]
mod device;
#[path = "../../../capsule_driver_nvme/src/setup/irq.rs"]
pub mod irq;
#[cfg(test)]
mod mask_tests;
#[path = "../../../capsule_driver_nvme/src/setup/mmio.rs"]
pub mod mmio;
#[path = "../../../capsule_driver_nvme/src/setup/pci.rs"]
pub mod pci;
#[path = "../../../capsule_driver_nvme/src/setup/power_off.rs"]
mod power_off;
#[cfg(test)]
mod refusal_tests;
pub mod say;
#[cfg(test)]
mod teardown_tests;
#[cfg(test)]
mod unmap_tests;

pub use device::Device;
