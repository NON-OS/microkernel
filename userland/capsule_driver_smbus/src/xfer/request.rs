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

//! One SMBus transfer as the wire asks for it. Pure, for the proofs.

use super::kind::Kind;

/// SMBus 2.0's largest block.
pub const BLOCK_MAX: usize = 32;
/// The address, flags, kind, command and length before a write's bytes.
pub const HEAD: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Xfer {
    /// The 7-bit address.
    pub addr: u8,
    pub read: bool,
    pub kind: Kind,
    pub cmd: u8,
    pub len: u8,
    pub data: [u8; BLOCK_MAX],
}

/// A request's payload: address, flags (bit 0 read), kind, command and
/// length, then the bytes a write carries: one for byte data, two (low
/// first) for word data, 1 to 32 for a block, none otherwise.
pub fn parse(p: &[u8]) -> Option<Xfer> {
    let (&[addr, flags, kind, cmd, len], rest) = p.split_first_chunk::<HEAD>()?;
    let (kind, read) = (Kind::from_wire(kind)?, flags & 1 != 0);
    if addr > 0x7F || flags & !1 != 0 {
        return None;
    }
    let carried = match (kind, read) {
        (Kind::Block, false) if (1..=BLOCK_MAX).contains(&(len as usize)) => len as usize,
        (Kind::Block, false) => return None,
        (Kind::WordData, false) => 2,
        (Kind::ByteData, false) => 1,
        _ => 0,
    };
    if len as usize != carried || rest.len() != carried {
        return None;
    }
    let mut data = [0u8; BLOCK_MAX];
    data[..carried].copy_from_slice(rest);
    Some(Xfer { addr, read, kind, cmd, len, data })
}
