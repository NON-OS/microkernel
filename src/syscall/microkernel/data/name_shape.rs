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

//! The shape every data file's name is held to, and the one name no
//! MkData call may take. No crate paths here, so host proofs mount this
//! file as it is.

/// The directory the person's files' objects are in (fs/blockfs_volume/user_obj).
pub(crate) const USER_ROOT: &[u8] = b"/u";

/// A slash, then 1 to 63 of [A-Za-z0-9._-], not starting with a dot: no
/// directories, so nothing under `/u/` or any other directory.
pub(crate) fn well_formed(name: &[u8]) -> bool {
    let ok = |b: &u8| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-');
    (2..=64).contains(&name.len()) && name[0] == b'/' && name[1] != b'.' && name[1..].iter().all(ok)
}

/// `/u` is a well-formed name, and is the directory of every object the
/// user-files broker keeps: a read would list them and a remove would take
/// them all. No MkData call, for any caller, names it.
pub(crate) fn is_user_root(name: &[u8]) -> bool {
    name == USER_ROOT
}
