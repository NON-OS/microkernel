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

//! Whether an mremap span is one mapping, as Linux requires.

use crate::linux::guest::{Guest, Region};

/// Every page of `[at, at + len)` is held with the same protection, backing
/// and provenance as `like`, which is what Linux keeps as one mapping.
pub(super) fn one_mapping(guest: &Guest, at: u64, len: u64, like: &Region) -> bool {
    let end = at + len;
    let mut reach = at;
    while reach < end {
        let Some(r) = guest.regions.iter().find(|r| r.at <= reach && reach < r.at + r.len) else {
            return false;
        };
        let same = (r.write, r.exec, r.access, r.backed, r.unproven)
            == (like.write, like.exec, like.access, like.backed, like.unproven);
        if !same {
            return false;
        }
        reach = r.at + r.len;
    }
    true
}
