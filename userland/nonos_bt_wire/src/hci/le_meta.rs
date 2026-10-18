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

//! The LE Meta event's subevents a central needs to open and secure a link
//! (Vol 4, Part E, 7.7.65): the first parameter octet is the subevent code.

use crate::addr::{AddrType, BdAddr};
use crate::cursor::Reader;
use crate::error::WireError;

/// The subevent code and its parameters.
pub fn split(params: &[u8]) -> Result<(u8, &[u8]), WireError> {
    let (code, rest) = params.split_first().ok_or(WireError::Short)?;
    Ok((*code, rest))
}

/// LE Connection Complete (Vol 4, Part E, 7.7.65.1).
pub struct LeConnectionComplete {
    pub status: u8,
    pub handle: u16,
    pub role: u8,
    pub peer_type: AddrType,
    pub peer: BdAddr,
    pub interval: u16,
    pub latency: u16,
    pub timeout: u16,
}

impl LeConnectionComplete {
    pub fn parse(sub: &[u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(sub);
        let (status, handle, role) = (r.u8()?, r.u16()? & 0x0fff, r.u8()?);
        let (peer_type, peer) = (AddrType::from_wire(r.u8()?)?, BdAddr(r.array()?));
        let (interval, latency, timeout) = (r.u16()?, r.u16()?, r.u16()?);
        let _central_clock_accuracy = r.u8()?;
        r.finish()?;
        Ok(Self { status, handle, role, peer_type, peer, interval, latency, timeout })
    }
}

/// LE Long Term Key Request (Vol 4, Part E, 7.7.65.5): the peripheral's
/// controller asks for the key to encrypt a link with.
pub struct LeLtkRequest {
    pub handle: u16,
    pub rand: [u8; 8],
    pub ediv: u16,
}

impl LeLtkRequest {
    pub fn parse(sub: &[u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(sub);
        let (handle, rand, ediv) = (r.u16()? & 0x0fff, r.array()?, r.u16()?);
        r.finish()?;
        Ok(Self { handle, rand, ediv })
    }
}
