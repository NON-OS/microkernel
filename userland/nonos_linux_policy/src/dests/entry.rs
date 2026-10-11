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

//! One destination of a list, read and checked.

use super::address::{v4, v6};
use super::error::DestsError;
use super::name;

/// kind, prefix, first port, last port, body length.
const HEAD: usize = 7;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    V4,
    V6,
    Name,
}

/// A destination: `body` is the address in network order, or the name.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dest<'a> {
    pub kind: Kind,
    pub prefix: u8,
    pub first_port: u16,
    pub last_port: u16,
    pub body: &'a [u8],
}

/// The entry `bytes` starts with, if it is valid, and its length.
pub(super) fn read(bytes: &[u8]) -> Result<(Dest<'_>, usize), DestsError> {
    let head = bytes.get(..HEAD).ok_or(DestsError::Short)?;
    let kind = match head[0] {
        4 => Kind::V4,
        6 => Kind::V6,
        b'N' => Kind::Name,
        _ => return Err(DestsError::Kind),
    };
    let len = HEAD + usize::from(head[6]);
    let dest = Dest {
        kind,
        prefix: head[1],
        first_port: u16::from_be_bytes([head[2], head[3]]),
        last_port: u16::from_be_bytes([head[4], head[5]]),
        body: bytes.get(HEAD..len).ok_or(DestsError::Short)?,
    };
    if dest.first_port == 0 || dest.first_port > dest.last_port {
        return Err(DestsError::Ports);
    }
    match kind {
        Kind::V4 => v4(dest.body, dest.prefix)?,
        Kind::V6 => v6(dest.body, dest.prefix)?,
        Kind::Name if dest.prefix == 0 && name::canonical(dest.body) => {}
        Kind::Name => return Err(DestsError::Name),
    }
    Ok((dest, len))
}
