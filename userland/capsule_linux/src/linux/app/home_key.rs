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

//! Which home a program's /home and /root are kept in, as the Lead ruled
//! (HANDOFF/catalogue/files-home-design.md): a pure function of the
//! listing it was sold under, so a new release keeps the home and two
//! listings never share one. Pure, so the host proofs hold it.
//!
//! The policy's listing id is the market's listing key, BLAKE3 of the
//! publisher's key and the listing's name (nonos_marketplace_abi::
//! listing_id): two publishers naming one listing get two homes.

pub const LISTED: &[u8] = b"NONOS.linux.home.v1";
/// A program no listing names: its home is its program's, for the boot.
pub const UNLISTED: &[u8] = b"NONOS.linux.home.unlisted.v1";

pub fn home_key(listing_key: &[u8; 32], program: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    match *listing_key == [0; 32] {
        false => h.update(LISTED).update(listing_key),
        true => h.update(UNLISTED).update(program),
    };
    *h.finalize().as_bytes()
}
