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

//! The bands' members a virtual machine adds (`priority`): its pointers on
//! the interactive path, its disk controllers on the storage path. Its
//! network cards are on the card table (registry/nics.rs) and its display on
//! the display list (registry/displays.rs), with the rest.

/// The pointer and keyboard paths in a virtual machine.
pub(super) const INTERACTIVE_VM: [&str; 4] =
    ["driver.virtio_input0", "driver.vmmdev0", "driver.vmmouse0", "driver.vmbus0"];

/// The disk controllers in a virtual machine: each polls a request out,
/// yielding and then sleeping a millisecond between looks.
pub(super) const STORAGE_PATH_VM: [&str; 5] = [
    "driver.piix_ide0",
    "driver.virtio_scsi0",
    "driver.pvscsi0",
    "driver.buslogic0",
    "driver.mpt0",
];
