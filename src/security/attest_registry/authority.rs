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

//! Whose policy tree proved a running capsule, for a gate that must know.

use super::table::TABLE;
use crate::security::dev_roots::Authority;

/// The authority that proved `pid`, or `None` when it passed no spawn gate.
pub fn authority_of(pid: u32) -> Option<Authority> {
    let t = TABLE.lock();
    t.entries[..t.used].iter().find(|e| e.pid == pid).map(|e| e.authority)
}
