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
pub mod efr;
#[cfg(feature = "nonos-iommu-amdvi")]
mod enable;
pub mod error;
pub mod event;
#[cfg(feature = "nonos-iommu-amdvi")]
mod flush;
pub mod identity_change;
mod init;
#[cfg(feature = "nonos-iommu-amdvi")]
pub mod intr;
pub mod intr_format;
pub mod intr_rule;
#[cfg(feature = "nonos-iommu-amdvi")]
mod log_line;
#[cfg(feature = "nonos-iommu-amdvi")]
mod log_ring;
#[cfg(feature = "nonos-iommu-amdvi")]
mod mmio;
#[cfg(feature = "nonos-iommu-amdvi")]
mod poll;
pub mod pte;
pub mod pte_child;
pub mod regs;
mod release;
#[cfg(feature = "nonos-iommu-amdvi")]
mod reserved;
#[cfg(feature = "nonos-iommu-amdvi")]
mod ring;
#[cfg(feature = "nonos-iommu-amdvi")]
mod submit;
#[cfg(feature = "nonos-iommu-amdvi")]
mod units;
#[cfg(feature = "nonos-iommu-amdvi")]
mod unnamed;
#[cfg(feature = "nonos-iommu-amdvi")]
mod visible;

pub use error::AmdViError;
pub use init::init;
pub use release::release_from_firmware;
#[cfg(feature = "nonos-iommu-amdvi")]
pub use {
    coverage::unit_covers,
    flush::{flush_device, flush_domain},
    poll::drain_events,
    units::{is_enforcing, units},
};
