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

//! What the endpoint answers. A SOCKS5 reply names no bound address
//! (0.0.0.0:0): the stream leaves from an exit, not from here. SOCKS4 has
//! one code for every refusal.

use alloc::vec::Vec;

use super::kinds::Ver;
use crate::linux::abi::errno::{EACCES, ECONNREFUSED, EHOSTUNREACH, ENETUNREACH, ETIMEDOUT};

pub const SUCCEEDED: u8 = 0;
pub const GENERAL_FAILURE: u8 = 1;
pub const NOT_ALLOWED: u8 = 2;
pub const NETWORK_UNREACHABLE: u8 = 3;
pub const HOST_UNREACHABLE: u8 = 4;
pub const CONNECTION_REFUSED: u8 = 5;
pub const COMMAND_NOT_SUPPORTED: u8 = 7;
pub const ADDRESS_TYPE_NOT_SUPPORTED: u8 = 8;

/// SOCKS4's request granted, and rejected or failed.
pub const GRANTED: u8 = 90;
pub const REJECTED: u8 = 91;

/// The method reply to a SOCKS5 greeting: no authentication, or none
/// acceptable, after which the endpoint closes.
pub fn method(no_auth: bool) -> [u8; 2] {
    [super::v5::VER, if no_auth { 0 } else { 0xFF }]
}

/// The reply to a request, in its version: SOCKS5 says the code, SOCKS4
/// says granted for success and rejected for anything else.
pub fn reply(ver: Ver, code: u8) -> Vec<u8> {
    match ver {
        Ver::Five => Vec::from([super::v5::VER, code, 0, 1, 0, 0, 0, 0, 0, 0]),
        Ver::Four => {
            Vec::from([0, if code == SUCCEEDED { GRANTED } else { REJECTED }, 0, 0, 0, 0, 0, 0])
        }
    }
}

/// The code for a stream that did not open with `errno`. EACCES is this
/// capsule's word for a destination the program's policy does not grant.
pub fn code_of(errno: i64) -> u8 {
    match errno {
        EACCES => NOT_ALLOWED,
        ENETUNREACH => NETWORK_UNREACHABLE,
        EHOSTUNREACH | ETIMEDOUT => HOST_UNREACHABLE,
        ECONNREFUSED => CONNECTION_REFUSED,
        _ => GENERAL_FAILURE,
    }
}
