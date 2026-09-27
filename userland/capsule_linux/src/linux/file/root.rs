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

//! The subtree the Linux world lives in, and the only name for it.

use alloc::vec::Vec;

/// Where the Linux world is kept. Every path a guest sees is relative
/// to this, and it never appears in anything handed back to a guest.
pub const ROOT: &[u8] = b"/linux";

/// A path in the store, already confined. Built only from a normalised
/// guest-visible path, by `resolve::key`.
/// `shared` is false for a path in the family's private directories.
pub struct Key(Vec<u8>, bool);

impl Key {
    /// `visible` must be absolute and free of `.` and `..`, which is what
    /// `resolve::visible` guarantees and the only thing that calls this.
    pub(super) fn under_root(visible: &[u8]) -> Key {
        // An install has no family, and writes only the shared tree.
        if !super::private::shared_writes_allowed() && super::private::is_private(visible) {
            let mut out = super::private::root();
            out.extend_from_slice(visible);
            return Key(out, false);
        }
        let mut out = Vec::with_capacity(ROOT.len() + visible.len());
        out.extend_from_slice(ROOT);
        /*
         * The guest's root is the store's `/linux`, not `/linux/`: a trailing
         * separator makes every listing prefix wrong by one byte and every
         * child look like a sibling of itself.
         */
        if visible != b"/" {
            out.extend_from_slice(visible);
        }
        Key(out, true)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// The shared tree is written by installs alone; a guest is refused.
    pub fn writable(&self) -> Result<(), &'static str> {
        match self.1 && !super::private::shared_writes_allowed() {
            true => Err("read-only file system"),
            false => Ok(()),
        }
    }
}
