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

//! A grant a depart took, marked (`acpi::depart`, through the release path).

use super::GONE;

/// The most recent marks kept; a grant id is never asked about long after.
const KEPT: usize = 256;

pub(in super::super) fn mark(pid: u32, grant_id: u64) {
    let mut g = GONE.lock();
    if g.len() == KEPT {
        g.pop_front();
    }
    g.push_back((pid, grant_id));
}
