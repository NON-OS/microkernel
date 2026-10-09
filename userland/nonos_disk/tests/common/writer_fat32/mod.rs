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

//! The writer's own FAT32 modules, from `src/fat32/`, mounted as `fat32`
//! at a test's root so their `crate::fat32` paths resolve: the parts with
//! no device behind them, which the crate keeps private.

#[path = "../../../src/fat32/bpb/mod.rs"]
pub mod bpb;
#[path = "../../../src/fat32/dir/mod.rs"]
pub mod dir;
#[path = "../../../src/fat32/geometry/mod.rs"]
pub mod geometry;
#[path = "../../../src/fat32/tree/mod.rs"]
pub mod tree;
#[path = "../../../src/fat32/write/mod.rs"]
pub mod write;
