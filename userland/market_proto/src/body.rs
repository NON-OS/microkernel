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

//! The bodies a client sends.

use alloc::vec::Vec;

/// One listing id, length-prefixed: what `OP_GET_APP` takes.
pub fn listing_body(listing: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + listing.len());
    body.extend_from_slice(&(listing.len() as u32).to_le_bytes());
    body.extend_from_slice(listing);
    body
}

/// A listing id and a release id, each length-prefixed: what
/// `OP_GET_RELEASE` and `OP_INSTALL_READY` take. An empty release names
/// the listing's default, which both operations resolve the same way.
pub fn pair_body(listing: &[u8], release: &[u8]) -> Vec<u8> {
    let mut body = listing_body(listing);
    body.extend_from_slice(&(release.len() as u32).to_le_bytes());
    body.extend_from_slice(release);
    body
}
