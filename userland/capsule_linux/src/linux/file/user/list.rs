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

//! Directory listings from the broker, and the folders in the home.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_user_files::errno::Errno;
use nonos_user_files::limits::LIST_MAX;

use crate::linux::guest::Guest;

use super::{link, place};

/// Every name in the broker directory `path`, after `.` and `..`, as
/// every Linux directory lists itself and its parent first.
pub(super) fn names(path: &[u8]) -> Result<Vec<String>, Errno> {
    let mut raw = Vec::new();
    let mut cookie = 0;
    loop {
        let got = link::with(|c| c.list(path, cookie, LIST_MAX as u32))?;
        raw.extend(got.entries.into_iter().map(|e| e.2));
        cookie = got.next;
        if got.end {
            break;
        }
    }
    let mut out = alloc::vec![String::from("."), String::from("..")];
    out.extend(super::open::strings(raw));
    Ok(out)
}

/// The home, listed for a guest served the folders, names both of them,
/// whatever the home's own tree holds.
pub fn home_names(guest: &Guest, path: &[u8], names: &mut Vec<String>) {
    if !place::is_home(guest, path) {
        return;
    }
    for folder in place::FOLDERS {
        let name = String::from_utf8_lossy(folder).into_owned();
        if !names.contains(&name) {
            names.push(name);
        }
    }
}
