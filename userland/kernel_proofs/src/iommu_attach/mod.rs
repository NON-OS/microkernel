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

//! An attach whose invalidation fails takes its context entry back out,
//! and a destroyed domain gives back its tables. The real steps and
//! walk are included by path.

#[path = "../../../../src/arch/x86_64/iommu/device/attach_steps.rs"]
pub mod attach_steps;
#[path = "../../../../src/arch/x86_64/iommu/tables/frames_of.rs"]
pub mod frames_of;
#[path = "../../../../src/arch/x86_64/iommu/backend/noted.rs"]
pub mod noted;
#[cfg(test)]
mod frames_tests;
#[cfg(test)]
mod noted_tests;
#[cfg(test)]
mod tests;
