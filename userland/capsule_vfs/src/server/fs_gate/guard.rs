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

//! The trees of the store only the Linux personality reaches.
//!
//! An app's home, /linux-home/<key>, and each run's private files,
//! /linux-private/<id>, hold what Linux apps keep: a wallet, a password
//! database, keys. vfs served them to any FileSystem holder, so the file
//! manager, or any capsule given the bit, could open, list or search them.
//! Now only a caller the kernel says holds ForeignExec reaches them, and
//! only capsule_linux holds it (fs_gate.rs). Listings, searches and the
//! recent-files journal never show them to anyone.

use alloc::vec::Vec;

/// The guarded trees, matched as whole path components.
pub const GUARDED: [&str; 2] = ["/linux-home", "/linux-private"];

/// Whether the normalized `path` is a guarded tree or inside one; never a
/// name that only begins with the same bytes, such as /linux-homes.
pub fn guarded(path: &str) -> bool {
    GUARDED.iter().any(|tree| match path.strip_prefix(tree) {
        Some(rest) => rest.is_empty() || rest.starts_with('/'),
        None => false,
    })
}

/// Whether the normalized `path` is guarded or holds a guarded tree, as the
/// root does: what a recursive copy or removal of it would reach.
pub fn touches(path: &str) -> bool {
    let above = |tree: &str| match tree.strip_prefix(path) {
        Some(rest) => path.ends_with('/') || rest.starts_with('/'),
        None => false,
    };
    guarded(path) || GUARDED.iter().any(|tree| above(tree))
}

/// A listing's `<u8 len><name>` entries without any in a guarded tree.
/// A malformed tail is dropped, as no listing vfs makes has one.
pub fn strip_listing(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len());
    let mut rest = body;
    while let Some((&len, tail)) = rest.split_first() {
        let Some(name) = tail.get(..len as usize) else { break };
        if !core::str::from_utf8(name).is_ok_and(guarded) {
            out.push(len);
            out.extend_from_slice(name);
        }
        rest = &tail[len as usize..];
    }
    out
}
