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

//! The rule alone, on bytes: which absolute, resolved guest paths are in
//! the person's folders, and the broker's name for each. No crate paths
//! here, so host proofs mount this file as it is.

/// The home the folders are in.
pub const HOME: &[u8] = b"/root";
/// The folders, as the broker names them and as they appear in the home.
pub const FOLDERS: [&[u8]; 5] = [b"Documents", b"Downloads", b"Music", b"Pictures", b"Videos"];

/// The broker's path for `visible` when it is in one of the folders:
/// `/root/Documents/a` is `Documents/a`. `visible` is absolute with `.`,
/// `..` and empty components already resolved (resolve::visible, walk).
pub fn broker_path(visible: &[u8]) -> Option<&[u8]> {
    let rest = visible.strip_prefix(HOME)?.strip_prefix(b"/")?;
    let top = rest.split(|b| *b == b'/').next()?;
    FOLDERS.contains(&top).then_some(rest)
}
