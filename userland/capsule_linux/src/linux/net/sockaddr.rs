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

//! `sockaddr_in` and `sockaddr_un` between guest memory and `Addr`.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::sock::Addr;

pub const AF_UNSPEC: u16 = 0;
pub const AF_UNIX: u16 = 1;
pub const AF_INET: u16 = 2;
pub const SOCKADDR_IN: usize = 16;

/// The family and address a guest named; EINVAL for one too short to hold
/// its family, EFAULT for one it cannot read.
pub fn read(guest: &Guest, at: u64, len: u64) -> Result<(u16, Addr), u64> {
    if len < 2 {
        return Err(errno::fail(errno::EINVAL));
    }
    let head = guest.read(at, 2).ok_or(errno::fail(errno::EFAULT))?;
    let family = u16::from_le_bytes([head[0], head[1]]);
    if family != AF_INET {
        return Ok((family, Addr::default()));
    }
    if len < SOCKADDR_IN as u64 {
        return Err(errno::fail(errno::EINVAL));
    }
    let raw = guest.read(at, SOCKADDR_IN).ok_or(errno::fail(errno::EFAULT))?;
    let port = u16::from_be_bytes([raw[2], raw[3]]);
    Ok((family, Addr { ip: [raw[4], raw[5], raw[6], raw[7]], port }))
}

/// True for 127.0.0.0/8, the only addresses the family keeps to itself.
pub fn is_loopback(ip: [u8; 4]) -> bool {
    ip[0] == 127
}
