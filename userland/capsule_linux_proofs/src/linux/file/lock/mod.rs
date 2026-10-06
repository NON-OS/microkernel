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

//! The family's lock table: a record, how a lock splits the ones it meets,
//! and how many the table holds. The table itself is one static the
//! capsule's serve loop alone touches, so the proofs use the pure parts.

#[allow(dead_code)]
#[path = "../../../../../capsule_linux/src/linux/file/locks/lock/apply.rs"]
pub mod apply;
#[path = "../../../../../capsule_linux/src/linux/file/locks/lock/room.rs"]
pub mod room;
#[allow(dead_code)]
#[path = "../../../../../capsule_linux/src/linux/file/locks/lock/table.rs"]
pub mod table;
