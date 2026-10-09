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

//! Intel Volume Management Device. With RST/VMD on in firmware the NVMe
//! drives are hidden in a private PCI domain behind the VMD endpoint; this
//! module finds each VMD, numbers and assigns the domain, gives its functions
//! a segment of their own, and routes their config space through CFGBAR.
//!
//! Children never interrupt: MSI from behind a VMD is remapped onto the VMD's
//! own vectors, which nothing here services, so their records carry no MSI-X
//! and the drivers poll. Their DMA reaches the IOMMU under the VMD's requester
//! id, which `dma_requester` gives the broker.

mod bar_size;
mod bring_up;
mod cfgbar;
pub mod config;
pub mod domain;
mod ensure;
mod enumerate;
mod membar;
mod probe;
mod registry;
mod report;

pub use enumerate::children;
pub use registry::{dma_requester, domain_count};
