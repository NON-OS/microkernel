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

//! SOCKS5, RFC 1928: the greeting, then the request. A command is refused
//! before its address is read, IPv6 before its sixteen bytes come.

use super::kinds::{name_ok, Connect, To};
use super::parsed::{Parsed, Refusal};

pub const VER: u8 = 5;
const NO_AUTH: u8 = 0;
const CONNECT: u8 = 1;
const ATYP_V4: u8 = 1;
const ATYP_NAME: u8 = 3;

/// The greeting: whether it offers no authentication, the one method
/// taken. One offering no method at all offers not that one.
pub fn greeting(buf: &[u8]) -> Parsed<bool> {
    match buf {
        [ver, ..] if *ver != VER => Parsed::Bad,
        [] | [_] => Parsed::Need,
        [_, n, methods @ ..] => match methods.get(..usize::from(*n)) {
            Some(m) => Parsed::Done(m.contains(&NO_AUTH), 2 + m.len()),
            None => Parsed::Need,
        },
    }
}

/// The request: VER CMD RSV ATYP, the address, the port in network order.
pub fn request(buf: &[u8]) -> Parsed<Connect> {
    match buf {
        [ver, ..] if *ver != VER => return Parsed::Bad,
        [_, cmd, ..] if *cmd != CONNECT => return Parsed::Refused(Refusal::Command),
        _ => {}
    }
    let [_, _, _, atyp, rest @ ..] = buf else {
        return Parsed::Need;
    };
    let (to, len) = match *atyp {
        ATYP_V4 => match rest.get(..4) {
            Some(&[a, b, c, d]) => (To::V4([a, b, c, d]), 4),
            _ => return Parsed::Need,
        },
        ATYP_NAME => match rest.split_first() {
            Some((&n, name)) => match name.get(..usize::from(n)) {
                Some(name) => (To::Name(name.to_vec()), 1 + name.len()),
                None => return Parsed::Need,
            },
            None => return Parsed::Need,
        },
        _ => return Parsed::Refused(Refusal::AddressType),
    };
    let Some(&[hi, lo]) = rest.get(len..len + 2) else {
        return Parsed::Need;
    };
    let port = u16::from_be_bytes([hi, lo]);
    let named_badly = matches!(&to, To::Name(n) if !name_ok(n));
    if port == 0 || named_badly || to == To::V4([0; 4]) {
        return Parsed::Refused(Refusal::Malformed);
    }
    Parsed::Done(Connect { to, port }, 4 + len + 2)
}
