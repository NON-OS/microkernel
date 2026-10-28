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

//! A refusal said on the debug line and named, for the families' installs.

use super::Why;

pub(super) fn refuse(line: &[u8], why: Why) -> Result<(), Why> {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    Err(why)
}

/// Which package and which check stopped the install, as "name-version
/// (check)", for the person who asked and no one else: init keeps it in
/// memory for this listing only, and it never reaches the debug line.
pub(super) fn tell(name: &str, version: &str, check: &str) {
    let line = match version.is_empty() {
        true => alloc::format!("{name} ({check})"),
        false => alloc::format!("{name}-{version} ({check})"),
    };
    let _ = nonos_libc::mk_app_install_tell(line.as_bytes());
}
