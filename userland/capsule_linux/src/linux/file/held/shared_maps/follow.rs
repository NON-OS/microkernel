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

//! A rename or an unlink of a file that may be mapped (names.rs).

use super::names::{rename, unlink};
use super::table::with;

/// `from` renamed `to`, a file or a directory, replacing what was there.
pub fn renamed(from: &[u8], to: &[u8]) {
    with(|all| rename(all, from, to));
}

/// The file at `path` unlinked, or replaced by a rename onto it.
pub fn unlinked(path: &[u8]) {
    with(|all| unlink(all, path));
}
