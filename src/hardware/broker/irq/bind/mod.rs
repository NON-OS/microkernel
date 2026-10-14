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

// x86-only: IO-APIC redirect (INTx) + PCI MSI-X capability program.
#![cfg(target_arch = "x86_64")]

mod bind;
mod grant_run;
mod handle_view;
mod intx;
mod intx_line;
mod intx_open;
mod intx_say;
mod message;
mod msi;
mod msi_layout;
mod msi_program;
mod msi_route;
mod msix;
mod msix_route;
mod quiet;
#[cfg(feature = "nonos-iommu-intremap")]
mod remap;
mod teardown;

pub use bind::bind;
pub(super) use message::release as release_irte;
pub(super) use teardown::{disable_msi_for_device, disable_msix_for_device, teardown_msix_vector};
