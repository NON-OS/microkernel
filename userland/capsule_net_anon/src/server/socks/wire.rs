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

//! The SOCKS5 greeting (RFC 1928 section 3), as bytes.

pub const VERSION: u8 = 5;
const METHOD_NO_AUTH: u8 = 0;
const METHOD_NONE_ACCEPTABLE: u8 = 0xFF;

/// A method from the private range (X'80' to X'FE') nonos_route_link lists
/// beside no-authentication for an install's download, never chosen in
/// reply: it only marks the stream as an install's, so it may take the warm
/// circuit (manager/warm_rule.rs). net.socks5 reads past it.
pub const METHOD_INSTALL: u8 = 0x80;

/// A whole greeting at the front of `buf`: whether no-authentication was
/// offered, whether the install mark was, and how many bytes it took. `None`
/// while it is incomplete. The methods are as many as the count byte says,
/// at most 255, and none is read past what has arrived.
pub fn greeting(buf: &[u8]) -> Option<Result<(bool, bool, usize), ()>> {
    let (&ver, rest) = buf.split_first()?;
    if ver != VERSION {
        return Some(Err(()));
    }
    let (&n, methods) = rest.split_first()?;
    let methods = methods.get(..n as usize)?;
    let install = methods.contains(&METHOD_INSTALL);
    Some(Ok((methods.contains(&METHOD_NO_AUTH), install, 2 + n as usize)))
}

/// The answer to a greeting.
pub fn method_reply(no_auth: bool) -> [u8; 2] {
    [VERSION, if no_auth { METHOD_NO_AUTH } else { METHOD_NONE_ACCEPTABLE }]
}
