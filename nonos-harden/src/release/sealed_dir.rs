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

//! A sealed directory (the ESP) against the record: exactly the files it
//! lists, each with its digest. A missing file, an extra one, or one whose
//! bytes changed is named.

use super::resolve::resolve;
use super::sealed::digest;
use super::walk::files;
use std::collections::BTreeSet;
use std::path::Path;

pub fn compare(
    root: &Path,
    rel: &str,
    want: &[(String, String)],
    bad: &mut Vec<String>,
) -> Result<(), String> {
    let dir = resolve(root, rel, true)?;
    let found = files(&dir, want.len())?;
    for (p, sha) in want {
        match found.binary_search(p) {
            Ok(_) => digest(&dir.join(p), sha, &format!("{rel}/{p}"), bad),
            Err(_) => bad.push(format!("{rel}/{p} is missing")),
        }
    }
    let listed: BTreeSet<&str> = want.iter().map(|(p, _)| p.as_str()).collect();
    let extra = found.iter().filter(|f| !listed.contains(f.as_str()));
    bad.extend(extra.map(|f| format!("{rel}/{f} is not in the record")));
    Ok(())
}
