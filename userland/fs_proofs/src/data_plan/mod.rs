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

//! The kernel's disk plan parser, assembled for the host. The plan is one
//! plain sector anyone with the disk can write, so every way it can lie is
//! tried here against the shipping source.

#[path = "../../../../src/fs/blockfs_volume/plan.rs"]
pub mod plan;
#[path = "../../../../src/fs/blockfs_volume/plan_types.rs"]
pub mod plan_types;

mod tests;
mod tests_random;
