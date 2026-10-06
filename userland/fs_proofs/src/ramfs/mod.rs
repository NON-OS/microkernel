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

// capsule_ramfs, driven through its own request handlers (the ramfs_host
// crate assembles them from capsule source). The kernel forwards a /ram
// file's offset, which lseek sets, and the length ftruncate names, so these
// are what any process can make ramfs size a file to.

mod tests_handles;
mod tests_limits;
mod tests_wire;
mod wire;
