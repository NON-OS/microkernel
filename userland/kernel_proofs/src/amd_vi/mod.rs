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
 * AMD-Vi: commands, device table entries, page table entries, event log
 * entries and register values, included by path and checked against AMD
 * IOMMU spec 48882 and the encodings in Linux drivers/iommu/amd.
 */

#[path = "../../../../src/arch/x86_64/amd_vi/command.rs"]
pub mod command;
#[path = "../../../../src/arch/x86_64/amd_vi/dte.rs"]
pub mod dte;
#[path = "../../../../src/arch/x86_64/amd_vi/event.rs"]
pub mod event;
#[path = "../../../../src/arch/x86_64/amd_vi/pte.rs"]
pub mod pte;
#[path = "../../../../src/arch/x86_64/amd_vi/regs.rs"]
pub mod regs;
mod command_tests;
mod table_tests;
