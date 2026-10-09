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

//! How many links the family's table holds. Each is up to two paths of this
//! capsule's memory, and every lookup walks the table, so it is bounded as a
//! small filesystem bounds its links: one more past the ceiling is ENOSPC.
//! Pure, so the host proofs hold it.

/// Far more than an image's own table and what its programs make.
pub const MAX_LINKS: usize = 16_384;

/// Whether a table of `len` links takes one more.
pub fn room(len: usize) -> bool {
    len < MAX_LINKS
}
