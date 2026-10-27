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

//! Where a package's pack and its trailer are in the store. Each family keeps
//! its packs in a directory of its own, so an Alpine and a Debian package of
//! one name are two packs: installing or removing one leaves the other, and
//! a run of one family loads none of another's.

use alloc::vec::Vec;

use crate::linux::file::family::Family;

/// Outside /linux and the families' roots: no guest path reaches it.
const PACKS: &[u8] = b"/linux-packs";
const PACK: &[u8] = b".pack";
/// The trailer's name, after the pack's own.
const TRAILER_AFTER: &[u8] = b".zk_trailer.bin";

/// The directory `family`'s packs are in. Each ends in `/`, so none is the
/// start of another's.
pub fn family_dir(family: Family) -> Vec<u8> {
    let name: &[u8] = match family {
        Family::Alpine => b"alpine",
        Family::Debian => b"deb",
        Family::Pacman => b"pacman",
    };
    [PACKS, b"/", name, b"/"].concat()
}

pub fn pack_path(family: Family, package: &str) -> Vec<u8> {
    [family_dir(family).as_slice(), package.as_bytes(), PACK].concat()
}

/// The package whose pack the store name `path` is, when it is one of
/// `family`'s packs, which a run of that family may load; a trailer, a `.new`
/// or another family's pack is none.
pub fn package_of(family: Family, path: &[u8]) -> Option<&[u8]> {
    path.strip_prefix(family_dir(family).as_slice())?.strip_suffix(PACK)
}

/// The trailer this machine minted over the pack at `pack`: over its index
/// (its first bytes, up to its data), which holds every file's digest.
pub fn trailer_of(pack: &[u8]) -> Vec<u8> {
    [pack, TRAILER_AFTER].concat()
}
