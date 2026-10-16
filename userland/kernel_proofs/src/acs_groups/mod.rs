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
 * Devices below one PCIe switch that reach each other past the IOMMU (L72):
 * the ACS controls a port needs, the extended capability walk, and the
 * groups no two capsules may share. The real rule is included by path.
 */

#[path = "../../../../src/arch/x86_64/iommu/acs/rule.rs"]
pub mod acs_rule;
#[path = "../../../../src/hardware/broker/confine/acs_span.rs"]
pub mod acs_span;
mod model;
mod model_tests;
mod span_tests;
mod tests;
