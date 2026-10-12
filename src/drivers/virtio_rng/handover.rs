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

//! The device goes to its capsule (L8). Two drivers once ran it at once:
//! the kernel kept polling a queue the capsule had just reset. Now the
//! broker asks here when anything claims a device, and if the kernel holds
//! it, the kernel stops counting it as a source, resets it and gives its
//! rings back before the claim goes on.

use core::sync::atomic::Ordering;

use super::state::{VIRTIO_RNG, VIRTIO_RNG_AVAILABLE};
use crate::drivers::pci::PciAddress;

/// True when the kernel held the device at `address` and has let it go.
pub fn hand_over(address: PciAddress) -> bool {
    let mut held = VIRTIO_RNG.lock();
    if !held.as_ref().is_some_and(|dev| dev.at(address)) {
        return false;
    }
    VIRTIO_RNG_AVAILABLE.store(false, Ordering::SeqCst);
    // Drop resets the device, then frees its rings.
    *held = None;
    crate::sys::serial::println(b"[VIRTIO-RNG] handed to its capsule");
    true
}
