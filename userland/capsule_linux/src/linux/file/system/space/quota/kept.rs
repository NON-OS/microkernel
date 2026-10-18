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

/* What the family keeps in its private directories, and what is left. */

use super::super::super::declared::PRIVATE_NAMES;

/* What the family keeps there: bytes, and names, a file or directory each. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kept {
    pub bytes: u64,
    pub names: u64,
}

impl Kept {
    /* `self` with `more` added, saturating: a count can only be refused. */
    pub fn plus(self, more: Kept) -> Kept {
        Kept {
            bytes: self.bytes.saturating_add(more.bytes),
            names: self.names.saturating_add(more.names),
        }
    }
}

/* The names left before the next one is refused, as statfs's f_ffree. */
pub fn names_free(now: Kept) -> u64 {
    PRIVATE_NAMES.saturating_sub(now.names)
}
