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

//! The family's table of shared file maps: the pure parts, as they ship.
//! The table itself is one static the capsule's serve loop alone touches,
//! and the peer copies in and out of a guest are the kernel's.

#[path = "../../../../../capsule_linux/src/linux/file/held/shared_maps/asks.rs"]
pub mod asks;
#[path = "../../../../../capsule_linux/src/linux/file/held/shared_maps/entry.rs"]
pub mod entry;
#[path = "../../../../../capsule_linux/src/linux/file/held/shared_maps/names.rs"]
pub mod names;
#[path = "../../../../../capsule_linux/src/linux/file/held/shared_maps/rules.rs"]
pub mod rules;
#[path = "../../../../../capsule_linux/src/linux/file/held/shared_maps/spans.rs"]
pub mod spans;
