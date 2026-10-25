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

//! A member that would take the place of one of the image's own files
//! (build/image_files.rs): /etc/ssl/cert.pem chooses the certificates every
//! guest in the tree trusts, /etc/passwd who a guest is, and /bin/qwenchat
//! what a tier runs. So would a link at a directory above one, which the
//! guest's path walk follows first. Pure, so the host proofs hold it.

/// Whether placing `at`, a member's path from the root with no `.`, `..`
/// or doubled slash, takes the place of a file in `image`, which is sorted;
/// for a link (`link`), at it or at any directory above it.
pub fn shadows(at: &[u8], link: bool, image: &[&[u8]]) -> bool {
    if image.binary_search(&at).is_ok() {
        return true;
    }
    // The root itself is a directory above every file.
    let dir = at.strip_suffix(b"/").unwrap_or(at);
    let under = |f: &&[u8]| f.len() > dir.len() && f.starts_with(dir) && f[dir.len()] == b'/';
    link && image.iter().any(under)
}
