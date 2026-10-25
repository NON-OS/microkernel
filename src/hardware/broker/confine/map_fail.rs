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
//! What a refused map leaves behind, so the grant's frames are freed only
//! when no device can reach them.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MapFail {
    /// Nothing of the grant is mapped: its frames may be freed.
    Freeable,
    /// The domain may still map the grant (a map whose flush failed, and an
    /// unmap that could not take it back): its frames must stay leaked and its
    /// IOVA reserved, as teardown does with a grant it cannot unmap.
    Pinned,
}

/// What a refused map leaves. `uncommitted`: the backend wrote its entries
/// and could not confirm the unit saw them. `unmap` is asked only then, and
/// says whether it took the range away. The IOVA goes back with the frames.
pub(crate) fn after_refusal(
    uncommitted: bool,
    unmap: impl FnOnce() -> bool,
) -> MapFail {
    if uncommitted && !unmap() {
        MapFail::Pinned
    } else {
        MapFail::Freeable
    }
}
