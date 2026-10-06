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

/*
 * The SOCKS5 a client speaks (RFC 1928): a greeting offering no
 * authentication, a CONNECT naming the host, and the two replies. The host
 * goes by name, so the exit resolves it and this machine never looks it up.
 * The replies are read from bytes another process sent, so every length is
 * checked before it is used and a short buffer means "more to come".
 */

use alloc::vec::Vec;

const VER: u8 = 5;
const METHOD_NONE: u8 = 0;
const CMD_CONNECT: u8 = 1;
const ATYP_IPV4: u8 = 1;
const ATYP_DOMAIN: u8 = 3;
const ATYP_IPV6: u8 = 4;

/* Version 5, one method, no authentication. */
pub const GREETING: [u8; 3] = [VER, 1, METHOD_NONE];

/* The reply code a CONNECT that worked carries. */
pub const REP_OK: u8 = 0;

/* What a reply at the front of a buffer came to. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Parsed<T> {
    /* Not all of it has arrived. */
    Need,
    /* The reply, and how many bytes it took. */
    Done(T, usize),
    /* Not a SOCKS5 reply. */
    Bad,
}

/* A CONNECT for `host` on `port`, or None for a name SOCKS cannot carry. */
pub fn connect_request(host: &str, port: u16) -> Option<Vec<u8>> {
    let name = host.as_bytes();
    let len = u8::try_from(name.len()).ok().filter(|&n| n > 0)?;
    let mut out = Vec::with_capacity(7 + name.len());
    out.extend_from_slice(&[VER, CMD_CONNECT, 0, ATYP_DOMAIN, len]);
    out.extend_from_slice(name);
    out.extend_from_slice(&port.to_be_bytes());
    Some(out)
}

/* The method reply: whether no authentication was accepted. */
pub fn method_reply(buf: &[u8]) -> Parsed<bool> {
    match buf {
        [] | [_] => Parsed::Need,
        [VER, method, ..] => Parsed::Done(*method == METHOD_NONE, 2),
        _ => Parsed::Bad,
    }
}

/* The CONNECT reply: its code, whatever address it bound. */
pub fn connect_reply(buf: &[u8]) -> Parsed<u8> {
    if buf.len() < 4 {
        return Parsed::Need;
    }
    if buf[0] != VER {
        return Parsed::Bad;
    }
    let len = match buf[3] {
        ATYP_IPV4 => 4 + 4 + 2,
        ATYP_IPV6 => 4 + 16 + 2,
        ATYP_DOMAIN => match buf.get(4) {
            Some(&n) => 4 + 1 + n as usize + 2,
            None => return Parsed::Need,
        },
        _ => return Parsed::Bad,
    };
    if buf.len() < len {
        return Parsed::Need;
    }
    Parsed::Done(buf[1], len)
}
