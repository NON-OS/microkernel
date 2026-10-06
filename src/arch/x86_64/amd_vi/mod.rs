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

//! AMD-Vi. Every unit is first taken back from firmware: a unit left
//! enabled translates every device's DMA through tables firmware built for
//! its own drivers, so the NVMe drive, the SATA controller and USB all fail
//! to reach the memory the kernel hands them; Linux disables such a unit
//! before anything else. A kernel built with `nonos-iommu-amdvi` then drives
//! the units (AMD IOMMU spec 48882): one shared device table, a command
//! buffer and an event log per unit, and capsule domains behind the same
//! broker calls as VT-d. Without the feature DMA on an AMD machine is
//! unrestricted and the posture line says so.

#[cfg(feature = "nonos-iommu-amdvi")]
mod bringup;
pub mod command;
mod control;
#[cfg(feature = "nonos-iommu-amdvi")]
mod coverage;
#[cfg(feature = "nonos-iommu-amdvi")]
mod devtab;
#[cfg(feature = "nonos-iommu-amdvi")]
pub mod domain;
pub mod dte;
#[cfg(feature = "nonos-iommu-amdvi")]
mod enable;
pub mod error;
pub mod event;
#[cfg(feature = "nonos-iommu-amdvi")]
mod flush;
mod init;
#[cfg(feature = "nonos-iommu-amdvi")]
mod log_line;
#[cfg(feature = "nonos-iommu-amdvi")]
mod log_ring;
#[cfg(feature = "nonos-iommu-amdvi")]
mod mmio;
#[cfg(feature = "nonos-iommu-amdvi")]
mod poll;
pub mod pte;
pub mod regs;
mod release;
#[cfg(feature = "nonos-iommu-amdvi")]
mod ring;
#[cfg(feature = "nonos-iommu-amdvi")]
mod submit;
#[cfg(feature = "nonos-iommu-amdvi")]
mod units;

#[cfg(feature = "nonos-iommu-amdvi")]
pub use coverage::unit_covers;
pub use error::AmdViError;
#[cfg(feature = "nonos-iommu-amdvi")]
pub use flush::{flush_device, flush_domain};
pub use init::init;
#[cfg(feature = "nonos-iommu-amdvi")]
pub use poll::drain_events;
pub use release::release_from_firmware;
#[cfg(feature = "nonos-iommu-amdvi")]
pub use units::{is_enforcing, units};
