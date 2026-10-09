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

//! Whether a run being given back is wholly handed out. A run freed twice,
//! or one that reaches over a free page, is refused whole: clearing it would
//! mark another grant's pages free, and the next map would hand them to a
//! second device while the first could still write them. Free of kernel
//! state so the host proofs include it unchanged.

use super::bitmap::WORD_BITS;

pub(crate) fn run_taken(used: &[u64], offset: usize, pages: usize) -> bool {
    pages != 0
        && (offset..offset + pages)
            .all(|page| used[page / WORD_BITS] & (1u64 << (page % WORD_BITS)) != 0)
}
