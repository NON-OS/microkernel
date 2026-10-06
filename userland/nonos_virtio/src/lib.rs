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

//! The virtio 1.0 ("modern") PCI transport, shared by the virtio drivers.
//!
//! A modern-only virtio function has no legacy I/O window. Its registers are
//! four structures (common, notify, ISR and device configuration) that
//! vendor capabilities in config space place inside a memory BAR. QEMU only
//! builds such functions once a device sits behind its IOMMU: with
//! iommu_platform=on it refuses legacy and transitional mode. A driver that
//! knows only the legacy layout takes the first MMIO BAR it finds, which on
//! such a function is the exclusive MSI-X table BAR, and the broker refuses
//! to map it.
//!
//! Everything about that transport that does not need the kernel is here:
//! the capability walk and its validation over a snapshot of config space,
//! the transport choice, feature negotiation, queue programming and the
//! notify address over a register trait, and bounds-checked volatile
//! accessors for the regions once mapped. The broker calls come in through
//! the `Broker` trait, which each driver implements on `nonos_libc`, so the
//! whole crate builds and is proved on the host.
//!
//! Modules name each other by their full `crate::` path, never through the
//! re-exports below, so the proofs crate can include them one by one.

#![no_std]

pub mod broker;
pub mod caps;
pub mod common;
pub mod error;
pub mod features;
pub mod map;
pub mod mmio;
pub mod notify;
pub mod pci;
pub mod select;

pub use broker::{Broker, MmioGrant};
pub use caps::{parse, ModernCaps, MsixLayout, Region};
pub use common::CommonCfg;
pub use error::VirtioError;
pub use map::{map_window, Need, Window};
pub use mmio::Mmio;
pub use pci::{BarInfo, BarKind, Bars, ConfigSpace};
pub use select::{choose, wants_probe, Kind};
