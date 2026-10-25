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

//! A running capsule's whole entry, for a record that must name it.

use super::entry::AttestedCapsule;
use super::table::TABLE;

/// What the spawn gate recorded for `pid`, or `None` when it passed none.
pub fn attested_of(pid: u32) -> Option<AttestedCapsule> {
    let t = TABLE.lock();
    t.entries[..t.used].iter().find(|e| e.pid == pid).copied()
}
