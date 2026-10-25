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

//! The L2CAP basic frame (Vol 3, Part A, 3.1): a 16-bit length of the
//! information payload, a 16-bit channel ID, then the payload, all
//! little-endian.

use alloc::vec::Vec;

use crate::cursor::{Reader, Writer};
use crate::error::WireError;

/// The channel ID and payload of one complete basic frame.
pub fn parse_basic(pdu: &[u8]) -> Result<(u16, &[u8]), WireError> {
    let mut r = Reader::new(pdu);
    let len = usize::from(r.u16()?);
    let cid = r.u16()?;
    let payload = r.bytes(len)?;
    r.finish()?;
    Ok((cid, payload))
}

pub fn basic(cid: u16, payload: &[u8]) -> Result<Vec<u8>, WireError> {
    let len = u16::try_from(payload.len()).map_err(|_| WireError::TooLong)?;
    let mut w = Writer::new();
    w.u16(len).u16(cid).bytes(payload);
    Ok(w.into_vec())
}
