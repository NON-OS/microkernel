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
//! Putting a package's files into the store, under the Linux root.

use super::auth::Verified;
use super::place_entry::one;
use super::program::record;
use super::tar::entries;

/// Unpack a package's authenticated files into the store and report how
/// many landed. `chosen` names the package the person asked for, whose
/// program is recorded so it can be started later.
pub fn unpack(files: &Verified, chosen: Option<&str>) -> usize {
    if let Some(name) = chosen {
        record(name, files);
    }
    entries(files.files()).iter().filter(|entry| one(entry)).count()
}
