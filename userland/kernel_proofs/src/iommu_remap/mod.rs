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
 * Interrupt remapping: the table address register, the remapped-format
 * entry, the remappable MSI message and the entry allocator, included by
 * path and checked against VT-d 3.4 chapters 5 and 9.10 and Linux
 * intel/irq_remapping.c.
 */

#[path = "../../../../src/arch/x86_64/iommu/remap/irte.rs"]
pub mod irte;
#[path = "../../../../src/arch/x86_64/iommu/remap/msi.rs"]
pub mod msi;
#[path = "../../../../src/arch/x86_64/iommu/regs/offsets/remap.rs"]
pub mod regs;
#[path = "../../../../src/arch/x86_64/iommu/remap/slots.rs"]
pub mod slots;
mod message_tests;
mod slot_tests;
mod tests;
