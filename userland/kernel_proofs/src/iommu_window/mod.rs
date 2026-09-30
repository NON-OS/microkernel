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
 * A remapping unit whose registers lie past the mapped page must be refused.
 *
 * The kernel's register decoders and registers_fit are included by path. The
 * probe mapped 4 KiB and then used IOTLB and fault-record offsets that CAP and
 * ECAP place up to 16 KiB in, with only a debug_assert on the window. The
 * checks below do not build against that code, which had no registers_fit.
 */

#[path = "../../../../src/arch/x86_64/iommu/regs/cap/fault.rs"]
pub mod cap;
#[path = "../../../../src/arch/x86_64/iommu/regs/offsets/invalidate.rs"]
pub mod offsets;
#[path = "../../../../src/arch/x86_64/iommu/regs/window.rs"]
pub mod window;
mod tests;
