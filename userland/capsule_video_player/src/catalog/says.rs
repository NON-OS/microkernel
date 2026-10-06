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

//! Why the library is empty when the folders were never read: a scan not yet
//! run, or a store that could not list them, is not a library with no videos.

/// The first failure of a scan in which no root could be listed, or `None`
/// when at least one root was read. `results` is each root's listing outcome.
pub fn scan_failure(results: &[Result<(), &'static str>]) -> Option<&'static str> {
    if results.iter().any(|r| r.is_ok()) {
        return None;
    }
    results.iter().find_map(|r| r.err())
}

/// The heading and note an empty library screen shows in place of its own
/// "no videos" words, or `None` when the folders were read and hold none.
pub fn library_unavailable(
    scanned: bool,
    scan_error: Option<&'static str>,
) -> Option<(&'static str, &'static str)> {
    if !scanned {
        return Some(("Looking for videos", "Reading Movies, Series, Downloads and Clips"));
    }
    match scan_error {
        Some("vfs ipc failed") => {
            Some(("Videos are not available", "The file store did not answer"))
        }
        Some(_) => Some(("Videos are not available", "The file store would not list your folders")),
        None => None,
    }
}

/// What the vfs client says when the store did not answer at all.
pub const SILENT: &str = "vfs ipc failed";

/// List the `N` roots in order with `list`, stopping at the first the store
/// does not answer: each such call waits out the five-second reply timeout on
/// the window's thread, and the scan made five of them and a probe per
/// video after. A root not asked reads as silent too.
pub fn list_roots<const N: usize>(
    mut list: impl FnMut(usize) -> Result<(), &'static str>,
) -> [Result<(), &'static str>; N] {
    let mut results = [Err(SILENT); N];
    for (i, result) in results.iter_mut().enumerate() {
        *result = list(i);
        if *result == Err(SILENT) {
            break;
        }
    }
    results
}
