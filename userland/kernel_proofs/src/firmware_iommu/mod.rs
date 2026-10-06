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

/*
 * Taking IOMMUs back from firmware. A VT-d unit with protected memory
 * regions or translation left on, or an AMD-Vi unit left enabled, blocks the
 * DMA every driver issues. The kernel's IVRS walk, the AMD control value that
 * stops a unit and the VT-d command that turns translation off are included
 * by path.
 */

#[path = "../../../../src/arch/x86_64/amd_vi/control.rs"]
#[allow(dead_code)]
pub mod amd_control;
/* Mounted once, in iommu_command; a second mount is clippy's duplicate_mod. */
pub use crate::iommu_command::{behaviour, global};
#[path = "../../../../src/arch/x86_64/acpi/parser/other/ivrs_walk.rs"]
pub mod ivrs_walk;
mod tests;
