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

//! The bodies a caller sends, and GET's and STATUS's as fetch reads them.

use alloc::vec::Vec;

use super::ops::{Put, CHUNK_MAX, GET_BODY};

pub fn get_body(listing_id: &[u8; 32], anchor: &[u8; 32], offset: u32) -> [u8; GET_BODY] {
    let mut b = [0u8; GET_BODY];
    b[..32].copy_from_slice(listing_id);
    b[32..64].copy_from_slice(anchor);
    b[64..].copy_from_slice(&offset.to_le_bytes());
    b
}

/// GET's listing, anchor and offset: exactly GET_BODY bytes, or nothing.
pub fn get_of(b: &[u8]) -> Option<([u8; 32], [u8; 32], u32)> {
    if b.len() != GET_BODY {
        return None;
    }
    let offset = u32::from_le_bytes(b[64..].try_into().ok()?);
    Some((b[..32].try_into().ok()?, b[32..64].try_into().ok()?, offset))
}

/// STATUS's anchor: exactly 32 bytes, or nothing.
pub fn anchor_of_body(b: &[u8]) -> Option<[u8; 32]> {
    b.try_into().ok()
}

/// Every PUT body for `container`, in order.
pub fn put_bodies(listing_id: &[u8; 32], container: &[u8]) -> Vec<Vec<u8>> {
    let total = container.len() as u32;
    let at = |i: usize| (i * CHUNK_MAX) as u32;
    let put = |(i, chunk)| Put { listing_id, total, offset: at(i), chunk }.encode();
    container.chunks(CHUNK_MAX).enumerate().map(put).collect()
}
