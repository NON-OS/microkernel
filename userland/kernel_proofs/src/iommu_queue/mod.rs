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
 * Queued invalidation: the descriptor encodings, the queue register values
 * and the ring arithmetic the kernel submits with, included by path and
 * checked against the bit positions of VT-d 3.4, section 6.5.2, and the
 * QI_* encodings in Linux include/linux/intel-iommu.h.
 */

#[path = "../../../../src/arch/x86_64/iommu/unit/queue/descriptor.rs"]
pub mod descriptor;
#[path = "../../../../src/arch/x86_64/iommu/regs/cap/drain.rs"]
pub mod drain;
#[path = "../../../../src/arch/x86_64/iommu/regs/cap/extended.rs"]
pub mod extended;
#[path = "../../../../src/arch/x86_64/iommu/regs/offsets/queue.rs"]
pub mod queue;
mod ring_tests;
mod tests;
