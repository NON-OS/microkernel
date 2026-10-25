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

//! One line of userland/capsule_linux/policies.txt: a vendor program's store
//! path, its services, and optionally its calls and destinations. The build
//! script and this crate's tests read it alike, so what the build accepts is
//! what they check.

use crate::mask::FILES_USER;
use crate::syscall_set::SyscallSet;

use super::dests_arg::dests_of;
use super::mask_names::bit_named;
use super::set_hex::set_of;

pub struct Line<'a> {
    pub store_path: &'a str,
    /// Where the file is, for a test guest's line; the vendor's are found in
    /// the Linux userland build.
    pub file: Option<&'a str>,
    pub mask: u64,
    pub calls: SyscallSet,
    /// The canonical destination list (src/dests), when the line names one.
    pub dests: Option<Vec<u8>>,
}

/// None for a blank line or a comment; the line's fault otherwise.
pub fn parse(line: &str) -> Result<Option<Line<'_>>, String> {
    let mut words = line.split_whitespace();
    let Some(store_path) = words.next().filter(|w| !w.starts_with('#')) else {
        return Ok(None);
    };
    if !store_path.starts_with("/bin/") || store_path.len() == 5 || store_path[5..].contains('/') {
        return Err(format!("{store_path}: not a program in /bin"));
    }
    let (mut mask, mut calls, mut file, mut dests) = (0, SyscallSet::ALL, None, None);
    for word in words {
        if let Some(path) = word.strip_prefix("file=").filter(|p| p.starts_with('/')) {
            file = Some(path);
            continue;
        }
        if let Some(hex) = word.strip_prefix("calls=") {
            calls = set_of(hex).ok_or_else(|| format!("{store_path}: calls= is not a set"))?;
            continue;
        }
        if let Some(text) = word.strip_prefix("dests=") {
            dests = Some(dests_of(text).map_err(|why| format!("{store_path}: {why}"))?);
            continue;
        }
        mask |=
            bit_named(word).ok_or_else(|| format!("{store_path}: no service is named {word}"))?;
    }
    // Nothing serves the person's files yet; a vendor row may not claim them.
    if mask & FILES_USER != 0 {
        return Err(format!("{store_path}: FILES_USER is reserved: nothing serves it yet"));
    }
    Ok(Some(Line { store_path, file, mask, calls, dests }))
}
