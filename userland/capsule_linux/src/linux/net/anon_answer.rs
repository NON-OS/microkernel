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

//! What a guest is told from net.anon's answers, in the terms connect_out.rs
//! uses for net.sockets': no transport, no circuit or no directory is
//! ENETUNREACH; a refusal is ECONNREFUSED; no answer, or one this capsule
//! cannot read, is EIO. A stream table full for this capsule is ENOBUFS.
//! Every reply is checked before it is believed, so a short, overlong or
//! unknown one ends in an errno and never in a panic.

use alloc::format;
use alloc::vec::Vec;

use crate::linux::abi::errno::{EAGAIN, ECONNREFUSED, EIO, ENETUNREACH, ENOBUFS, EPIPE};

use super::anon_ops::{
    E_BAD_LEN, E_DIRECTORY_STALE, E_NO_CIRCUIT, E_NO_DIRECTORY, E_NO_LINK, E_NO_PATH, E_NO_STREAM,
    E_NO_TCP, E_OK, E_RX_EMPTY, E_STREAM_CLOSED, E_TABLE_FULL, E_WOULD_BLOCK, PAYLOAD_MAX,
};
use super::sock::Got;

/// The longest legal domain name, as host_body.rs bounds a name for
/// net.sockets.
const MAX_HOST: usize = 253;

/// One reply: its status and body, or None when none came or it was not a
/// reply (anon_wire.rs).
pub type Reply<'a> = Option<(u16, &'a [u8])>;

/// An open's body: the port, then the host. A host a BEGIN cannot carry as
/// written (empty, too long, or holding the ':' or NUL that end its part of
/// "host:port") is None.
pub fn open_body(host: &[u8], port: u16) -> Option<Vec<u8>> {
    if host.is_empty() || host.len() > MAX_HOST || host.iter().any(|&b| b == b':' || b == 0) {
        return None;
    }
    let mut body = Vec::with_capacity(2 + host.len());
    body.extend_from_slice(&port.to_le_bytes());
    body.extend_from_slice(host);
    Some(body)
}

/// A literal address as net.anon is given it: the dotted quad, so the exit
/// connects to it and nothing here looks a name up.
pub fn dotted(ip: [u8; 4]) -> Vec<u8> {
    let [a, b, c, d] = ip;
    format!("{a}.{b}.{c}.{d}").into_bytes()
}

/// The stream id an open was given, or the errno a connect answers.
pub fn opened(reply: Reply) -> Result<u16, i64> {
    match reply {
        Some((E_OK, &[lo, hi])) => Ok(u16::from_le_bytes([lo, hi])),
        Some((E_OK, _)) => Err(EIO),
        /*
         * net.anon has no directory, no relay to build through, no link to its
         * guard, or no circuit: there is no route out, which a tool has to
         * read as unreachable, not as a peer that refused.
         */
        Some((
            E_NO_TCP | E_NO_DIRECTORY | E_DIRECTORY_STALE | E_NO_PATH | E_NO_LINK | E_NO_CIRCUIT,
            _,
        )) => Err(ENETUNREACH),
        /*
         * This capsule, one caller for every guest, holds as many streams as
         * net.anon gives one caller (half its table). ENOBUFS fails the
         * connect at once and says why; EAGAIN would park a blocking connect
         * with nothing to wake it, and a non-blocking one reads it as a
         * connect in progress.
         */
        Some((E_TABLE_FULL, _)) => Err(ENOBUFS),
        Some((E_BAD_LEN, _)) => Err(ECONNREFUSED),
        Some(_) | None => Err(EIO),
    }
}

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
