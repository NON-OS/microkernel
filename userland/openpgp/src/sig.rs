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

//! A v4 signature packet (RFC 9580 5.2.3).

use alloc::vec::Vec;

use super::mpi::values;
use super::packet::next;
use super::refusal::Refusal;
use super::subpacket::{read, Found};

const SIGNATURE: u8 = 2;

pub struct Signature<'a> {
    pub kind: u8,
    pub algo: u8,
    pub hash: u8,
    /// Version through the end of the hashed area: what the digest covers.
    pub hashed: &'a [u8],
    pub issuer: Found,
    pub left16: [u8; 2],
    pub values: Vec<&'a [u8]>,
}

/// A detached signature file holds exactly one signature packet.
pub fn signature(file: &[u8]) -> Result<Signature<'_>, Refusal> {
    let (p, rest) = next(file).ok_or(Refusal::Malformed)?;
    if p.tag != SIGNATURE || !rest.is_empty() {
        return Err(Refusal::Malformed);
    }
    let b = p.body;
    if b.first() != Some(&4) {
        return Err(Refusal::Version);
    }
    let len = |at: usize| b.get(at..at + 2).map(|x| usize::from(u16::from_be_bytes([x[0], x[1]])));
    let hlen = len(4).ok_or(Refusal::Malformed)?;
    let hashed_end = 6 + hlen;
    let ulen = len(hashed_end).ok_or(Refusal::Malformed)?;
    let unhashed_end = hashed_end + 2 + ulen;
    let mut issuer = Found::default();
    read(b.get(6..hashed_end).ok_or(Refusal::Malformed)?, true, &mut issuer)?;
    read(b.get(hashed_end + 2..unhashed_end).ok_or(Refusal::Malformed)?, false, &mut issuer)?;
    let left = b.get(unhashed_end..unhashed_end + 2).ok_or(Refusal::Malformed)?;
    let values = values(b[2], &b[unhashed_end + 2..])?;
    Ok(Signature {
        kind: b[1],
        algo: b[2],
        hash: b[3],
        hashed: &b[..hashed_end],
        issuer,
        left16: [left[0], left[1]],
        values,
    })
}
