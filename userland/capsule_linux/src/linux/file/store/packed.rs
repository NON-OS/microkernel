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

//! Where the store's own tree has no file, the packs may: the shared tree's
//! keys answered out of them (pack/).

use super::super::root::Key;
use super::bytes::HashFault;

/// The guest-visible path of a key in the shared tree, which is where the
/// packs' files are; None for a family's private key.
pub(super) fn packed(at: &Key) -> Option<&[u8]> {
    match (at.1, &at.0[at.2..]) {
        (false, _) => None,
        (true, b"") => Some(b"/"),
        (true, v) => Some(v),
    }
}

/// A packed program's hash: its bytes are read whole to be checked anyway.
pub(super) fn packed_hash(at: &Key, max: u64) -> Option<Result<[u8; 32], HashFault>> {
    let bytes = super::pack::read(packed(at)?, u32::MAX)?.map_err(HashFault::Unread);
    Some(bytes.and_then(|b| match b.len() as u64 > max {
        true => Err(HashFault::Unread("larger than any program this installs")),
        false => Ok(*blake3::hash(&b).as_bytes()),
    }))
}

/// Whether the packs hold a file or directory at `at`.
pub fn packed_found(at: &Key) -> bool {
    packed_stat(at).is_some()
}

/// A packed file's size and whether it is a directory, as the store says it.
pub(super) fn packed_stat(at: &Key) -> Option<(u64, bool)> {
    super::pack::stat(packed(at)?)
}
