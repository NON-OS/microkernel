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

//! SOCKS5 reply codes, and the reply a CONNECT gets.

use super::wire::VERSION;

pub const REP_OK: u8 = 0;
pub const REP_FAILURE: u8 = 1;
pub const REP_NOT_ALLOWED: u8 = 2;
pub const REP_NET_UNREACHABLE: u8 = 3;
pub const REP_HOST_UNREACHABLE: u8 = 4;
pub const REP_REFUSED: u8 = 5;
pub const REP_TTL_EXPIRED: u8 = 6;
pub const REP_CMD_UNSUPPORTED: u8 = 7;
pub const REP_ATYP_UNSUPPORTED: u8 = 8;

/// A CONNECT reply with code `rep` and an all-zero IPv4 bound address.
pub fn reply(rep: u8) -> [u8; 10] {
    [VERSION, rep, 0, 1, 0, 0, 0, 0, 0, 0]
}

/// The SOCKS reply for a stream the exit ended before it connected, from
/// the END reason (tor-spec 6.3).
pub fn rep_for_end(reason: u8) -> u8 {
    match reason {
        2 => REP_HOST_UNREACHABLE, /* RESOLVEFAILED */
        3 => REP_REFUSED,          /* CONNECTREFUSED */
        4 => REP_NOT_ALLOWED,      /* EXITPOLICY */
        7 => REP_TTL_EXPIRED,      /* TIMEOUT */
        8 => REP_NET_UNREACHABLE,  /* NOROUTE */
        _ => REP_FAILURE,
    }
}
