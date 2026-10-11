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

//! Every other package's manifest, and a manifest kept only while it has
//! something left to take out.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::super::manifest::{encode, Manifest};
use super::one::{at, dir, read};

/// Every other package's manifest, so what two packages share stays.
pub(in crate::linux::install) fn others(name: &str) -> Vec<Manifest> {
    let listed = vfs::list_paths(mk_getpid(), &dir()).unwrap_or_default();
    let names = listed.iter().map(|p| p.rsplit('/').next().unwrap_or(p)).map(String::from);
    names.filter(|n| !n.is_empty() && n != name).filter_map(|n| read(&n)).collect()
}

/// Keep only what is still to take out, or drop the record once nothing is.
pub(in crate::linux::install) fn settle(name: &str, left: &Manifest) -> bool {
    let pid = mk_getpid();
    if left.files.is_empty() && left.links.is_empty() {
        return vfs::unlink(pid, &at(name)).is_ok();
    }
    vfs::write_file(pid, &at(name), &encode(left)).is_ok()
}
