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

//! The TPM2 table's start methods, as the TCG ACPI specification numbers
//! them (recalled: unverified). This kernel drives FIFO and both CRB
//! forms; ACPI start (AMD's firmware TPM) rings its doorbell through a
//! _DSM, which is AML, and Arm SMC start through firmware calls.

pub const START_ACPI: u32 = 2;
pub const START_FIFO: u32 = 6;
pub const START_CRB: u32 = 7;
pub const START_CRB_ACPI: u32 = 8;
