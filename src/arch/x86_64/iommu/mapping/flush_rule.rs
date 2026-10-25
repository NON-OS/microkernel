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

//! What a change to a domain's leaf entries owes the units' caches. Pure, so
//! the host proofs hold it.
//!
//! Without caching mode a unit never caches an entry that grants nothing
//! (VT-d 3.4, 6.1), so a map, which only fills empty entries, needs no
//! invalidation at all; the write buffer is still drained where CAP.RWBF says
//! there is one. Under caching mode, as QEMU and hypervisors run it, empty
//! entries are cached too and a map is invalidated like an unmap. Either way
//! only the domain whose entries changed is named, never every domain.

/// Whether a commit has to invalidate the domain's IOTLB entries.
pub(super) const fn invalidates(mapped: bool, caching_mode: bool) -> bool {
    !mapped || caching_mode
}
