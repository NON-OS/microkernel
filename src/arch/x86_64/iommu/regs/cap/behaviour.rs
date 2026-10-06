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

/// Units that set this need an explicit flush after a table write before the
/// hardware is guaranteed to see it.
pub const fn requires_write_buffer_flush(cap: u64) -> bool {
    cap & (1 << 4) != 0
}

/// When set the unit caches not-present entries, so any change that creates a
/// mapping must be followed by an invalidation.
pub const fn caching_mode(cap: u64) -> bool {
    cap & (1 << 7) != 0
}

/// ECAP.C: the unit snoops CPU caches on a table walk. When clear, a table
/// write sits in a cache line the hardware never reads until it is flushed.
pub const fn page_walk_coherent(ecap: u64) -> bool {
    ecap & 1 != 0
}

/// CAP.PLMR or CAP.PHMR: the unit has protected memory regions, and PMEN is
/// a register it implements.
pub const fn has_protected_regions(cap: u64) -> bool {
    cap & ((1 << 5) | (1 << 6)) != 0
}
