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

//! struct msghdr, as sendmsg and recvmsg find it in guest memory.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::iov;

/// struct msghdr on x86_64.
const MSGHDR: usize = 56;
pub const NAMELEN_AT: u64 = 8;
pub const CONTROLLEN_AT: u64 = 40;
pub const FLAGS_AT: u64 = 48;

pub struct Hdr {
    pub name: u64,
    pub namelen: u64,
    pub iov: iov::Iov,
    pub controllen: u64,
}

pub fn hdr(guest: &Guest, msg: u64) -> Result<Hdr, u64> {
    let raw = guest.read(msg, MSGHDR).ok_or(errno::fail(errno::EFAULT))?;
    let word = |i: usize| u64::from_le_bytes(raw[i..i + 8].try_into().unwrap_or([0; 8]));
    Ok(Hdr {
        name: word(0),
        namelen: u64::from(u32::from_le_bytes([raw[8], raw[9], raw[10], raw[11]])),
        iov: iov::read(guest, word(16), word(24))?,
        controllen: word(40),
    })
}
