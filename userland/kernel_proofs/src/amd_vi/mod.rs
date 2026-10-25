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
 * entries, register values and remapped interrupts, included by path and
 * checked against the layouts of AMD IOMMU spec 48882.
 */

#[path = "../../../../src/arch/x86_64/iommu/amd_vi/command.rs"]
pub mod command;
mod command_tests;
#[path = "../../../../src/arch/x86_64/iommu/amd_vi/dte.rs"]
pub mod dte;
#[path = "../../../../src/arch/x86_64/iommu/amd_vi/event.rs"]
pub mod event;
#[path = "../../../../src/arch/x86_64/iommu/amd_vi/identity_change.rs"]
pub mod identity_change;
mod identity_tests;
#[path = "../../../../src/arch/x86_64/iommu/amd_vi/intr_format.rs"]
pub mod intr_format;
#[path = "../../../../src/arch/x86_64/iommu/amd_vi/intr_rule.rs"]
pub mod intr_rule;
mod intr_rule_tests;
mod intr_slot_tests;
mod intr_tests;
#[path = "../../../../src/arch/x86_64/iommu/amd_vi/pte.rs"]
pub mod pte;
#[path = "../../../../src/arch/x86_64/iommu/amd_vi/pte_child.rs"]
pub mod pte_child;
#[path = "../../../../src/arch/x86_64/iommu/amd_vi/regs.rs"]
pub mod regs;
#[path = "../../../../src/arch/x86_64/iommu/amd_vi/intr/slots.rs"]
pub mod slots;
mod table_tests;
