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

//! `MkIrqBind` with `BIND_MSI` or `BIND_MSIX` on aarch64: vectors from the
//! MSI controller the device tree names, the function programmed to write
//! them to its doorbell, which the capsule's IOMMU domain maps for it.
//!
//! The capability layout, its programming and the quieting of the other
//! kind are the x86 broker's files, mounted here: config space is the same
//! on both arches, only the message differs.

#[path = "../../bind/handle_view.rs"]
mod handle_view;
#[path = "../../bind/msi_layout.rs"]
mod msi_layout;
#[path = "../../bind/msi_program.rs"]
mod msi_program;
#[path = "../../bind/quiet.rs"]
mod quiet;

mod bind;
mod close;
mod entries;
mod open;
mod program;
mod record;
mod release;
mod silence;

pub(super) use bind::bind;
pub(super) use release::{release_where, unmap_vector};
