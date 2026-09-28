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

//! The RELAY_BEGIN body that asks an exit to open a connection.

extern crate alloc;

use alloc::vec::Vec;

/*
 * tor-spec section 6.2: ADDRPORT is a nul terminated "host:port" string,
 * followed by four optional flag bytes.
 *
 * IPV4_NOT_OK is deliberately not set and IPV6_OK is, so an exit may answer with
 * either family. Refusing IPv6 would narrow which exits can serve a name for no
 * benefit to a client that never sees the address.
 */
const FLAG_IPV6_OK: u32 = 1;

/// Body for a BEGIN to `host` on `port`. `None` if the host is too long to fit a
/// relay message alongside its flags.
pub fn body(host: &[u8], port: u16) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(host.len() + 12);
    out.extend_from_slice(host);
    out.push(b':');
    let mut digits = [0u8; 5];
    let width = decimal(&mut digits, port);
    out.extend_from_slice(&digits[..width]);
    out.push(0);
    out.extend_from_slice(&FLAG_IPV6_OK.to_be_bytes());
    if out.len() > crate::cell::RELAY_BODY_BYTES {
        return None;
    }
    Some(out)
}

fn decimal(out: &mut [u8; 5], value: u16) -> usize {
    let mut scratch = [0u8; 5];
    let mut count = 0usize;
    let mut left = value;
    loop {
        scratch[count] = b'0' + (left % 10) as u8;
        left /= 10;
        count += 1;
        if left == 0 {
            break;
        }
    }
    for index in 0..count {
        out[index] = scratch[count - 1 - index];
    }
    count
}
