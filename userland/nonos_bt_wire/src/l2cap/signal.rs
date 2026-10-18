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

//! LE signaling commands (Vol 3, Part A, 4): code, identifier, a 16-bit
//! data length, then the data, one command per frame on the LE signaling
//! channel. Identifier 0 is never valid.

use crate::cursor::{Reader, Writer};
use crate::error::WireError;
use alloc::vec::Vec;

/// Vol 3, Part A, 4.1.
pub const COMMAND_REJECT: u8 = 0x01;
/// Vol 3, Part A, 4.6 and 4.7.
pub const DISCONNECTION_REQUEST: u8 = 0x06;
pub const DISCONNECTION_RESPONSE: u8 = 0x07;
/// Vol 3, Part A, 4.20 and 4.21.
pub const CONN_PARAM_UPDATE_REQUEST: u8 = 0x12;
pub const CONN_PARAM_UPDATE_RESPONSE: u8 = 0x13;
/// Vol 3, Part A, 4.22.
pub const LE_CREDIT_CONNECTION_REQUEST: u8 = 0x14;

/// Command Reject reasons (Vol 3, Part A, 4.1).
pub const NOT_UNDERSTOOD: u16 = 0x0000;
pub const SIGNAL_MTU_EXCEEDED: u16 = 0x0001;
pub const INVALID_CID: u16 = 0x0002;

#[derive(Debug, PartialEq, Eq)]
pub struct Signal<'a> {
    pub code: u8,
    pub id: u8,
    pub data: &'a [u8],
}

/// The one command in an LE signaling frame.
pub fn parse_signal(frame: &[u8]) -> Result<Signal<'_>, WireError> {
    let mut r = Reader::new(frame);
    let (code, id) = (r.u8()?, r.u8()?);
    let len = usize::from(r.u16()?);
    let data = r.bytes(len)?;
    r.finish()?;
    if id == 0 {
        return Err(WireError::Reserved);
    }
    Ok(Signal { code, id, data })
}

pub fn signal(code: u8, id: u8, data: &[u8]) -> Result<Vec<u8>, WireError> {
    let len = u16::try_from(data.len()).map_err(|_| WireError::TooLong)?;
    if id == 0 {
        return Err(WireError::Reserved);
    }
    let mut w = Writer::new();
    w.u8(code).u8(id).u16(len).bytes(data);
    Ok(w.into_vec())
}

/// Command Reject for `id`, with a reason that carries no data.
pub fn command_reject(id: u8, reason: u16) -> Result<Vec<u8>, WireError> {
    signal(COMMAND_REJECT, id, &reason.to_le_bytes())
}
