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

//! `sockaddr_un` as a guest names it: a path, an abstract name, or only the
//! family, which asks bind for a name of Linux's choosing.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::net::sockaddr::{self, AF_UNIX};

/// sun_family and the 108 bytes of sun_path.
const SOCKADDR_UN: u64 = 110;

pub enum UAddr {
    Auto,
    Path(Vec<u8>),
    Abstract(Vec<u8>),
}

pub fn read(guest: &Guest, at: u64, len: u64) -> Result<UAddr, u64> {
    if !(2..=SOCKADDR_UN).contains(&len) {
        return Err(errno::fail(errno::EINVAL));
    }
    let raw = guest.read(at, len as usize).ok_or(errno::fail(errno::EFAULT))?;
    let path = &raw[2..];
    Ok(match path.first() {
        None => UAddr::Auto,
        Some(0) => UAddr::Abstract(path[1..].to_vec()),
        /* A path ends at its first NUL, however long the guest said it was. */
        Some(_) => {
            let end = path.iter().position(|&b| b == 0).unwrap_or(path.len());
            UAddr::Path(path[..end].to_vec())
        }
    })
}

/// The name a bind or connect gives: EINVAL for another family.
pub fn unix_addr(guest: &Guest, at: u64, len: u64) -> Result<UAddr, u64> {
    match sockaddr::read(guest, at, len)? {
        (AF_UNIX, _) => read(guest, at, len),
        _ => Err(errno::fail(errno::EINVAL)),
    }
}
