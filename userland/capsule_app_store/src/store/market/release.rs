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

//! The release a listing would install: which version, from where, and what
//! the operator recorded when it checked the bytes.

use alloc::vec::Vec;

use super::wire::exchange;

const OP_GET_RELEASE: u16 = 4;

pub struct Release {
    pub version: Vec<u8>,
    /// The operator's validation note, such as the size it hashed.
    pub note: Vec<u8>,
}

/// The default release: an empty id asks for it.
pub fn fetch(port: u32, request_id: u32, listing: &[u8]) -> Option<Release> {
    let mut body = Vec::with_capacity(8 + listing.len());
    body.extend_from_slice(&(listing.len() as u32).to_le_bytes());
    body.extend_from_slice(listing);
    body.extend_from_slice(&0u32.to_le_bytes());
    let out = exchange(port, OP_GET_RELEASE, request_id, &body).ok()?;
    // release_id, manifest, package, url, signature, arches, abi, caps, status, note
    let (version, at) = lp(&out, 0)?;
    // The url is provenance the installer checks; the store does not show it.
    let (_, mut at) = lp(&out, at + 64)?;
    at = skip_blob(&out, at)?;
    at = skip_list(&out, at)? + 4;
    at = skip_list(&out, at)? + 1;
    let (note, _) = lp(&out, at)?;
    Some(Release { version, note })
}

fn lp(body: &[u8], at: usize) -> Option<(Vec<u8>, usize)> {
    let len = u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?) as usize;
    let end = (at + 4).checked_add(len)?;
    Some((body.get(at + 4..end)?.to_vec(), end))
}

fn skip_blob(body: &[u8], at: usize) -> Option<usize> {
    lp(body, at).map(|(_, end)| end)
}

/// A count, then that many length-prefixed strings.
fn skip_list(body: &[u8], at: usize) -> Option<usize> {
    let n = u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?);
    (0..n).try_fold(at + 4, |at, _| skip_blob(body, at))
}
