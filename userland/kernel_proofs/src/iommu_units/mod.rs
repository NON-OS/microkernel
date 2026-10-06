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
 * Every remapping unit is pointed at one set of tables, so the tables may use
 * only what all units support. The kernel's capability decoders and the fold
 * over a set of units are included by path.
 *
 * Before this, bring-up programmed the first unit DMAR listed. On a client
 * Intel machine that is the graphics unit, so the disk and USB controllers,
 * behind the second unit, were handed device addresses nothing translated.
 */

#[path = "../../../../src/arch/x86_64/iommu/regs/cap/agaw.rs"]
pub mod agaw;
#[path = "../../../../src/arch/x86_64/iommu/regs/cap/limits.rs"]
pub mod limits;
#[path = "../../../../src/arch/x86_64/iommu/regs/cap/shared.rs"]
pub mod shared;
mod tests;
