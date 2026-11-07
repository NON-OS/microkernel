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

//! RFC 1929: the username and password a SOCKS5 client sends once it has
//! chosen that method. SimpleX sends one to ask Tor for a circuit of its
//! own. Here the pair is read whole and let go: it is not kept, logged or
//! routed by, and every pair is taken, as Tor's own SOCKS port takes any.
//! It names no destination and grants nothing; the CONNECT that follows
//! is held to the program's policy as one without it is.

use super::parsed::Parsed;

pub const VER: u8 = 1;
const SUCCEEDED: u8 = 0;
const FAILED: u8 = 1;

/// VER ULEN UNAME PLEN PASSWD: how many bytes the pair took. Either may be
/// empty; SimpleX sends an empty password. Each length is one byte, so the
/// whole is at most 513 bytes.
pub fn credentials(buf: &[u8]) -> Parsed<()> {
    let [ver, ulen, rest @ ..] = buf else {
        return match buf.first() {
            Some(&ver) if ver != VER => Parsed::Bad,
            _ => Parsed::Need,
        };
    };
    if *ver != VER {
        return Parsed::Bad;
    }
    let ulen = usize::from(*ulen);
    let Some(&plen) = rest.get(ulen) else {
        return Parsed::Need;
    };
    let plen = usize::from(plen);
    match ulen.checked_add(1).and_then(|at| at.checked_add(plen)) {
        Some(end) if end <= rest.len() => Parsed::Done((), 2 + end),
        _ => Parsed::Need,
    }
}

/// The answer to a pair: taken, or refused, after which the endpoint closes.
pub fn status(taken: bool) -> [u8; 2] {
    [VER, if taken { SUCCEEDED } else { FAILED }]
}
