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
//! The rest of the broker's names, as the rest of the kernel uses them:
//! the tables, the boot's registrations and the arch-specific stand-ins
//! (exports.rs holds the claim and grant calls).

#[cfg(not(target_arch = "x86_64"))]
pub use super::pio_absent::{
    pio_release_all_for_pid, pio_release_for_device as pio_release_for_device_unkeyed,
};
#[cfg(not(target_arch = "x86_64"))]
pub use super::pio_absent_claim::pio_release_for_device;
pub use super::platform::register_legacy as register_legacy_platform_devices;
pub use super::table::{
    contains, external_port_of, init_from_pci, list, list_by_class, list_shown,
};
pub use super::vmbus::{register_vmbus, vmbus_held_by, VMBUS_DEVICE, VMBUS_VENDOR};
pub use super::windows::touches_device_window;
