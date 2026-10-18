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

//! What a guest is told of bytes sent and taken through net.anon.

use super::super::anon_ops::{
    E_NO_CIRCUIT, E_NO_LINK, E_NO_STREAM, E_OK, E_RX_EMPTY, E_STREAM_CLOSED, E_WOULD_BLOCK,
    PAYLOAD_MAX,
};
use super::super::sock::Got;
use super::open::Reply;
use crate::linux::abi::errno::{EAGAIN, EIO, EPIPE};

/// The count a send of `asked` bytes moved, or its errno. A window that is
/// shut is EAGAIN, which a blocking caller waits on; a stream net.anon has
/// ended or does not hold is EPIPE.
pub fn sent(reply: Reply, asked: usize) -> Result<usize, i64> {
    match reply {
        Some((E_OK, &[a, b, c, d])) => {
            let n = usize::try_from(u32::from_le_bytes([a, b, c, d])).map_err(|_| EIO)?;
            match n {
                _ if n > asked => Err(EIO),
                0 if asked != 0 => Err(EAGAIN),
                n => Ok(n),
            }
        }
        Some((E_OK, _)) => Err(EIO),
        Some((E_WOULD_BLOCK, &[])) => Err(EAGAIN),
        Some((E_STREAM_CLOSED | E_NO_STREAM | E_NO_LINK | E_NO_CIRCUIT, _)) => Err(EPIPE),
        Some(_) | None => Err(EIO),
    }
}

/// What a read from net.anon brought.
pub fn got(reply: Reply) -> Got {
    match reply {
        Some((E_OK, b)) if !b.is_empty() && b.len() <= PAYLOAD_MAX => Got::Bytes(b.to_vec()),
        Some((E_RX_EMPTY, &[])) => Got::Nothing,
        /* The END reason, whether another exit is needed, whether clean. */
        Some((E_STREAM_CLOSED, &[reason, _, _])) => Got::End(reason),
        Some((E_NO_STREAM, _)) => Got::Gone,
        Some(_) => Got::Garbled,
        None => Got::Silent,
    }
}
