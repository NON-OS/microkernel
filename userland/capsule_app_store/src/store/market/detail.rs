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

//! Everything about one listing that the list reply leaves out.

use alloc::vec::Vec;

use super::wire::call;

const OP_GET_APP: u16 = 3;

pub struct Detail {
    pub publisher: Vec<u8>,
    pub description: Vec<u8>,
}

pub fn fetch(port: u32, request_id: u32, listing: &[u8]) -> Option<Detail> {
    let mut body = Vec::with_capacity(4 + listing.len());
    body.extend_from_slice(&(listing.len() as u32).to_le_bytes());
    body.extend_from_slice(listing);
    let out = call(port, OP_GET_APP, request_id, &body)?;

    // listing_id, capsule_id, name, publisher, pubkey, description, count
    let (_, at) = lp(&out, 0)?;
    let at = at + 32;
    let (_, at) = lp(&out, at)?;
    let (publisher, at) = lp(&out, at)?;
    let at = at + 32;
    let (description, at) = lp(&out, at)?;
    // The release count closes the message.
    out.get(at..at + 4)?;
    Some(Detail { publisher, description })
}

/// Four bytes of length then the bytes, every bound checked against the
/// buffer that arrived rather than the length claiming to describe it.
fn lp(body: &[u8], at: usize) -> Option<(Vec<u8>, usize)> {
    let len = u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?) as usize;
    let start = at + 4;
    let end = start.checked_add(len)?;
    Some((body.get(start..end)?.to_vec(), end))
}
