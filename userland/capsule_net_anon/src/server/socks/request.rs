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

//! The SOCKS5 CONNECT request (RFC 1928 section 4), as bytes.

extern crate alloc;

use alloc::vec::Vec;

use super::rep::{REP_ATYP_UNSUPPORTED, REP_CMD_UNSUPPORTED, REP_FAILURE};
use super::wire::VERSION;

const CMD_CONNECT: u8 = 1;
const ATYP_IPV4: u8 = 1;
const ATYP_DOMAIN: u8 = 3;

/// The host to reach, its port, and the bytes the request took.
type Request = (Vec<u8>, u16, usize);

/// A CONNECT request at the front of `buf`, as (host, port, bytes taken).
/// `None` while incomplete; `Err(rep)` for a request this front refuses,
/// with the reply code to refuse it with. Names are handed to the exit to
/// resolve, so no lookup happens on this machine. IPv6 is refused: an exit
/// is asked for it by name or not at all.
pub fn connect(buf: &[u8]) -> Option<Result<Request, u8>> {
    if buf.len() < 4 {
        return None;
    }
    if buf[0] != VERSION {
        return Some(Err(REP_FAILURE));
    }
    if buf[1] != CMD_CONNECT {
        return Some(Err(REP_CMD_UNSUPPORTED));
    }
    let (host, at) = match buf[3] {
        ATYP_IPV4 => {
            let a = buf.get(4..8)?;
            let text = alloc::format!("{}.{}.{}.{}", a[0], a[1], a[2], a[3]);
            (text.into_bytes(), 8)
        }
        ATYP_DOMAIN => {
            let n = *buf.get(4)? as usize;
            if n == 0 {
                return Some(Err(REP_FAILURE));
            }
            (buf.get(5..5 + n)?.to_vec(), 5 + n)
        }
        _ => return Some(Err(REP_ATYP_UNSUPPORTED)),
    };
    let p = buf.get(at..at + 2)?;
    Some(Ok((host, u16::from_be_bytes([p[0], p[1]]), at + 2)))
}
