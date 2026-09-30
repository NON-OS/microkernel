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

//! The kernel's own readers of the disk plan and the key header, compiled
//! for the host from `src/fs/blockfs_volume/`, so the tests read what the
//! installer wrote with the code that reads it at boot.

#[path = "../../../../../src/fs/blockfs_volume/key_header.rs"]
mod key_header;
#[path = "../../../../../src/fs/blockfs_volume/plan.rs"]
mod plan;
#[path = "../../../../../src/fs/blockfs_volume/plan_types.rs"]
mod plan_types;
mod reach;

pub use reach::{keyed_by, plan_of, KeyedBy, PlanError, KERNEL_AAD_END, KERNEL_KEY_LBA};
