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

//! Device drivers start only for hardware the machine has.
//!
//! Every storage driver is in the image, because which disk a machine keeps
//! NONOS on is not known when the image is built, and the network driver is
//! in it for the machines that have its device. A driver without its device
//! is refused by name rather than started: the virtio drivers poll for their
//! device without end, and each spawn costs an attestation the boot waits
//! for.

use crate::hardware::inventory::{present as has, HardwareFamily};
use crate::sys::boot_log;

pub(super) fn present(prefix: &str, family: HardwareFamily) -> bool {
    if has(family) {
        return true;
    }
    boot_log::ok(prefix, "no controller present, not spawned");
    false
}
