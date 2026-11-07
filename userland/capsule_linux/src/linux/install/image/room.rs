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

//! Whether one member can be placed in the heap left. Its stored bytes are
//! cut out of its chunks (the chunks and the cut, each at most its raw
//! length, since a member is stored as xz only when that is smaller), then
//! decompressed beside them, then dropped; its tar and the pack built from
//! it are each about its raw length (fetch/room.rs). So the most held at
//! once is twice the raw length, the spare fetch/room.rs keeps, and the one
//! chunk the reader keeps. Asked before any of it is read, so a member too
//! large for this machine's heap ends the install as UNPACK_MEMORY, naming
//! it, never as a fault partway. Pure, so the host proofs hold it, and
//! hold that the largest member the pack tool writes fits the install
//! role's heap (heap.rs INSTALL_HEAP).

const SPARE_MIN: u64 = 1 << 20;

/// What placing a member of `raw` bytes takes, read in chunks of `chunk`.
pub fn needs(raw: u64, chunk: u32) -> u64 {
    raw.saturating_mul(2).saturating_add((raw >> 3).max(SPARE_MIN)).saturating_add(u64::from(chunk))
}
