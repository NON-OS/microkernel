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

//! Whether one package can be placed in the heap left: its files inflated,
//! and its pack built beside them, are each about its installed size (the
//! tar walk borrows its members from the first, tar_kind.rs). Asked before
//! any of it is inflated, so a package too large for this machine's install
//! heap ends the install as UNPACK_MEMORY, naming it, and never as a fault
//! partway (Telegram's, 134 on an 8 GB laptop). Pure, so the host proofs
//! hold it.

/// Beside the two copies: the member table, the trailers it mints and the
/// paths it records, an eighth of the package and a MiB.
const SPARE_SHIFT: u32 = 3;
const SPARE_MIN: usize = 1 << 20;

/// What placing a package of `installed` bytes takes; 0 when the index
/// gave no size, which the fallible inflate and build still bound.
pub fn needs(installed: usize) -> usize {
    match installed {
        0 => 0,
        n => n.saturating_mul(2).saturating_add((n >> SPARE_SHIFT).max(SPARE_MIN)),
    }
}
