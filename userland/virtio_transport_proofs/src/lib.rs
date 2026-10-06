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

//! The shared virtio 1.0 PCI transport, proved without a device.
//!
//! Each `#[path]` pulls in the shipping source of `nonos_virtio`, module by
//! module at the same place in the crate, so these tests run the code the
//! virtio-net, virtio-blk, virtio-rng and virtio-gpu capsules boot with.
//!
//! What is held to the specification here, and to the kernel's broker:
//! the capability walk over synthetic config spaces (QEMU's modern-only
//! layout, and hostile ones: duplicates, truncated capabilities, I/O and
//! absent BARs, pointer loops, pointers out of range, capabilities over the
//! MSI-X table); the transport choice and the register window it maps,
//! which for a modern-only function is never the MSI-X BAR; feature
//! negotiation; the bring-up order and queue programming against a model
//! device; and the notify arithmetic and the bounds of every register
//! access.

#[path = "../../nonos_virtio/src/broker.rs"]
pub mod broker;
#[path = "../../nonos_virtio/src/caps/mod.rs"]
pub mod caps;
#[path = "../../nonos_virtio/src/common/mod.rs"]
pub mod common;
#[path = "../../nonos_virtio/src/error.rs"]
pub mod error;
#[path = "../../nonos_virtio/src/features.rs"]
pub mod features;
#[path = "../../nonos_virtio/src/map/mod.rs"]
pub mod map;
#[path = "../../nonos_virtio/src/mmio.rs"]
pub mod mmio;
#[path = "../../nonos_virtio/src/notify.rs"]
pub mod notify;
#[path = "../../nonos_virtio/src/pci/mod.rs"]
pub mod pci;
#[path = "../../nonos_virtio/src/select.rs"]
pub mod select;

#[cfg(test)]
mod tests;
