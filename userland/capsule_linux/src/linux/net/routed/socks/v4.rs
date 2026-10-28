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

//! SOCKS4 and SOCKS4a: VN CD DSTPORT DSTIP USERID NUL, and for 4a, a
//! DSTIP of 0.0.0.x with x not 0, then the name and a NUL. The user id is
//! read past and never used. A 4a name is the name, never looked up.

use super::kinds::{name_ok, Connect, To};
use super::parsed::{Parsed, Refusal};

pub const VER: u8 = 4;
const CONNECT: u8 = 1;

/// One SOCKS4 or SOCKS4a request.
pub fn request(buf: &[u8]) -> Parsed<Connect> {
    match buf {
        [ver, ..] if *ver != VER => return Parsed::Bad,
        [_, cmd, ..] if *cmd != CONNECT => return Parsed::Refused(Refusal::Command),
        _ => {}
    }
    let [_, _, hi, lo, a, b, c, d, rest @ ..] = buf else {
        return Parsed::Need;
    };
    let (port, ip) = (u16::from_be_bytes([*hi, *lo]), [*a, *b, *c, *d]);
    let Some(user) = rest.iter().position(|&x| x == 0) else {
        return Parsed::Need;
    };
    let after = 8 + user + 1;
    if port == 0 || ip == [0; 4] {
        return Parsed::Refused(Refusal::Malformed);
    }
    if ip[..3] != [0, 0, 0] {
        return Parsed::Done(Connect { to: To::V4(ip), port }, after);
    }
    let tail = &buf[after..];
    let Some(len) = tail.iter().position(|&x| x == 0) else {
        return Parsed::Need;
    };
    match &tail[..len] {
        name if name_ok(name) => {
            Parsed::Done(Connect { to: To::Name(name.to_vec()), port }, after + len + 1)
        }
        _ => Parsed::Refused(Refusal::Malformed),
    }
}
