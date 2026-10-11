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

//! Whether a list of operation codes has no repeat, in a const context.

/// O(n^2) over a few dozen entries, in a const context, so it costs nothing
/// at runtime and the build fails on a repeat.
pub const fn all_distinct(ops: &[u16]) -> bool {
    let mut i = 0;
    while i < ops.len() {
        let mut j = i + 1;
        while j < ops.len() {
            if ops[i] == ops[j] {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}
