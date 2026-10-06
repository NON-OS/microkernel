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
 * The load average's module shape: system/load/ reads its constants from a
 * sibling `declared`, as it does in the capsule, and so does system/space/'s
 * quota on the private directories.
 */

#[path = "../../../capsule_linux/src/linux/file/system/declared/mod.rs"]
pub mod declared;

#[path = "../../../capsule_linux/src/linux/file/system/load/mod.rs"]
pub mod load;

pub mod machine;
pub mod space;
