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

/// A whole greeting at the front of `buf`: whether no-authentication was
/// offered, and how many bytes it took. `None` while it is incomplete.
pub fn greeting(buf: &[u8]) -> Option<Result<(bool, usize), ()>> {
    let (&ver, rest) = buf.split_first()?;
    if ver != VERSION {
        return Some(Err(()));
    }
    let (&n, methods) = rest.split_first()?;
    let methods = methods.get(..n as usize)?;
    Some(Ok((methods.contains(&METHOD_NO_AUTH), 2 + n as usize)))
}

/// The answer to a greeting.
pub fn method_reply(no_auth: bool) -> [u8; 2] {
    [VERSION, if no_auth { METHOD_NO_AUTH } else { METHOD_NONE_ACCEPTABLE }]
}
