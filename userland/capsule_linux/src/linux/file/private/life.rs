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

//! A family's private directories, made before it runs and gone after.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::names::{choose, root, PRIVATE};

/// A fresh id and the scratch directories a program expects to find. False
/// when there is no id to keep them apart by, and the guest must not start.
pub fn prepare() -> bool {
    if !choose() {
        return false;
    }
    let pid = mk_getpid();
    for p in PRIVATE {
        let mut at = root();
        at.extend_from_slice(p);
        if vfs::mkdir(pid, &at).is_err() {
            return false;
        }
    }
    true
}

/// Everything the family wrote to its own directories, removed.
pub fn clear() {
    let _ = vfs::rmdir(mk_getpid(), &root(), true);
}
