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

//! The two calls a stream outside the family makes to net.sockets: a
//! mixnet socket, and a connect to an address or to the name this capsule
//! invented it for.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::call::call;
use super::dns::host_for;
use super::ops::{DOMAIN, KIND_MIXNET, OP_CONNECT, OP_CONNECT_HOST, OP_SOCKET};
use super::sock::Addr;

pub fn open() -> Result<u32, u64> {
    let mut body = Vec::with_capacity(4);
    body.extend_from_slice(&DOMAIN.to_le_bytes());
    body.extend_from_slice(&KIND_MIXNET.to_le_bytes());
    match call(OP_SOCKET, &body, 8) {
        Some((0, out)) if out.len() >= 4 => {
            Ok(u32::from_le_bytes([out[0], out[1], out[2], out[3]]))
        }
        Some(_) => Err(errno::fail(errno::ENOMEM)),
        None => Err(errno::fail(errno::EIO)),
    }
}

/// The service's answer to a connect; None when it did not answer.
pub fn dial(guest: &Guest, handle: u32, to: Addr) -> Result<Option<(u16, Vec<u8>)>, u64> {
    /* An address this capsule invented for a name goes back to being the name. */
    if let Some(host) = host_for(guest, to.ip) {
        let body = super::host_body::host_body(handle, to.port, &host)
            .ok_or(errno::fail(errno::EINVAL))?;
        return Ok(call(OP_CONNECT_HOST, &body, 0));
    }
    let mut body = Vec::with_capacity(10);
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(&to.ip);
    body.extend_from_slice(&to.port.to_le_bytes());
    Ok(call(OP_CONNECT, &body, 0))
}
