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

//! What a name in a pack may be: a plain path under the package's root,
//! with nothing a resolver could read as leaving it.

use crate::format::NAME_MAX;

pub fn valid(name: &[u8]) -> bool {
    !name.is_empty()
        && name.len() <= NAME_MAX
        && !name.contains(&0)
        && name.split(|b| *b == b'/').all(|c| !c.is_empty() && c != b"." && c != b"..")
}
