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

//! Whether this boot has a store to keep anything in.

use nonos_app_skeleton::clients::vfs;

/// True once vfs has loaded the store from a NONOS disk without error. A boot
/// with no such disk settles with a nonzero status and so reads false.
pub fn store_ready() -> bool {
    vfs::store_settled() == Ok(true) && vfs::store_status() == Ok(0)
}
