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

//! Where a package's pack and its trailer are in the store.

use alloc::vec::Vec;

/// Outside /linux and the families' roots: no guest path reaches it.
pub const PACKS: &[u8] = b"/linux-packs";
pub const PACK: &[u8] = b".pack";
/// The trailer's name, after the pack's own.
pub const TRAILER_AFTER: &[u8] = b".zk_trailer.bin";

pub fn pack_path(package: &str) -> Vec<u8> {
    [PACKS, b"/", package.as_bytes(), PACK].concat()
}

/// The trailer this machine minted over the pack's index (its first bytes,
/// up to its data), which holds every file's digest.
pub fn trailer_path(package: &str) -> Vec<u8> {
    [pack_path(package).as_slice(), TRAILER_AFTER].concat()
}
