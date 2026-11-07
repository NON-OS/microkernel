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

use crate::memory::paging::types::PagePermissions;

/// Where a capsule sees the page: above the mmap window
/// (`process::mmap_va`, which ends at 0x7000_0000_0000) and below the user
/// stack, so no allocation of the process's own is ever placed on it.
pub const VA: u64 = 0x0000_7FFF_0000_0000;

/// Read and user: present, user, not writable, no-execute
/// (`PagePermissions::to_pte_flags`). Never W, never X.
pub const PERMS: PagePermissions = PagePermissions::user_ro();
