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

//! What a guest's connect is told when its Nym stream did not open. Route
//! link says why in a sentence (refusal.rs), one per SOCKS reply code and
//! one per way the proxy can fail; each is read back here against the very
//! sentence it gives, never a copy. The exit's rules refusing the
//! destination, or the destination refusing, is ECONNREFUSED; no exit for
//! it, EHOSTUNREACH; the request expiring on its way, ETIMEDOUT; anything
//! else is no route out, ENETUNREACH, so a tool reads it as the network and
//! not as the host.

use super::super::Proxy;
use crate::linux::abi::errno::{ECONNREFUSED, EHOSTUNREACH, ENETUNREACH, ETIMEDOUT};

/// The SOCKS reply codes net.socks5 gives a CONNECT, RFC 1928.
const NOT_ALLOWED: u8 = 2;
const HOST_UNREACHABLE: u8 = 4;
const REFUSED: u8 = 5;
const EXPIRED: u8 = 6;

pub fn errno_of(why: &str) -> i64 {
    let said = |rep: u8| Proxy::Nym.refused(rep) == why;
    if said(NOT_ALLOWED) || said(REFUSED) {
        ECONNREFUSED
    } else if said(HOST_UNREACHABLE) {
        EHOSTUNREACH
    } else if said(EXPIRED) {
        ETIMEDOUT
    } else {
        ENETUNREACH
    }
}
