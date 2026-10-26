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

//! What admitted a capsule, as its PCR 14 event's kind byte names it, and
//! the verifier digest beside it. Only a STARK proof under the vendor's
//! root is kind 1: anything else a release would refuse has its own kind.

/// Its STARK proof, under the vendor's root.
pub const VENDOR: u8 = 1;
/// Its Merkle path alone, on a development image.
pub const PATH_ONLY: u8 = 2;
/// Its STARK proof, under a root a person enrolled on this machine.
pub const DEVELOPER: u8 = 3;
/// This machine's own tag on a local build, with no proof behind it.
pub const LOCAL_BUILD: u8 = 4;
/// Hashed in place of a trailer for a path-only capsule, and for no other.
pub const PATH_ONLY_TAG: &[u8] = b"nonos capsule path-only";

/// The kind, from how the trailer was taken: a path or a local tag says so
/// whatever root took it, and only the vendor's root makes kind 1.
pub fn kind(path_only: bool, local: bool, vendor: bool) -> u8 {
    match (path_only, local, vendor) {
        (true, _, _) => PATH_ONLY,
        (_, true, _) => LOCAL_BUILD,
        (_, _, true) => VENDOR,
        _ => DEVELOPER,
    }
}

/// The verifier's digest: the tag's for a path-only capsule, else the
/// trailer's.
pub fn verifier(trailer: &[u8], kind: u8) -> [u8; 32] {
    match kind {
        PATH_ONLY => crate::crypto::hash::sha256(PATH_ONLY_TAG),
        _ => crate::crypto::hash::sha256(trailer),
    }
}
