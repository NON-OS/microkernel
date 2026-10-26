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

//! The test's own reading of DDC/CI, from the message layout as this test
//! reads it and apart from the driver: requests, a Get VCP reply's rule, a
//! capabilities fragment's rule (taken off the bus: `ci_bus.rs`).

use crate::ddc::error::DdcError;
use crate::ddc::reply::Reading;

fn xor(seed: u8, b: &[u8]) -> u8 {
    b.iter().fold(seed, |a, v| a ^ v)
}

/// A request: source 0x51, a length byte, the message, a checksum over all
/// of it and the display's write address 0x6E.
pub fn request(msg: &[u8]) -> Vec<u8> {
    let mut r = vec![0x51, 0x80 | msg.len() as u8];
    r.extend_from_slice(msg);
    r.push(xor(0x6E, &r));
    r
}

/// A Get VCP reply, read whole or refused for the first thing wrong.
pub fn vcp(code: u8, r: &[u8]) -> Result<Reading, DdcError> {
    match (r[0], r[1]) {
        (0x6E, 0x80) if xor(0x50, &r[..3]) == 0 => return Err(DdcError::NotOffered),
        (0x6E, 0x80) => return Err(DdcError::Checksum),
        (0x6E, 0x88) => {}
        (0x6E, _) => return Err(DdcError::Length),
        _ => return Err(DdcError::Reply),
    }
    match (xor(0x50, &r[..11]), r[2], r[3], r[4]) {
        (1.., ..) => Err(DdcError::Checksum),
        (0, 0x02, 0, c) if c == code => Ok(Reading {
            max: u16::from_be_bytes([r[6], r[7]]),
            present: u16::from_be_bytes([r[8], r[9]]),
        }),
        (0, 0x02, 1, c) if c == code => Err(DdcError::NotOffered),
        _ => Err(DdcError::Reply),
    }
}

/// A capabilities reply's string bytes for `offset`: the display's
/// address, a length of 3 to 35 with the top bit, the checksum, opcode
/// 0xE3 and the offset echoed; None for anything else.
pub fn fragment(offset: u16, r: &[u8]) -> Option<Vec<u8>> {
    let n = (r[1] & 0x7F) as usize;
    let framed = r[0] == 0x6E && r[1] & 0x80 != 0 && (3..=35).contains(&n);
    let ok = framed && xor(0x50, &r[..3 + n]) == 0 && r[2] == 0xE3;
    (ok && u16::from_be_bytes([r[3], r[4]]) == offset).then(|| r[5..2 + n].to_vec())
}
