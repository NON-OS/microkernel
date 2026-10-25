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

//! A bound lease's line, pure, so dhcp_proofs holds every byte of it:
//!
//!   [DHCP] lease 10.0.2.15/24 gw 10.0.2.2 from 10.0.2.2
//!
//! A boot reads receive off it: a lease binds only once the server's offer
//! and ack both came in through the card. An address the server left out
//! prints `none`, never `0.0.0.0`, which a reader would take for a route.

/// The longest line: three 255.255.255.255 and /32, 72 bytes.
pub const LINE_MAX: usize = "[DHCP] lease /32 gw  from \n".len() + 3 * 15;

fn push(out: &mut [u8; LINE_MAX], len: &mut usize, bytes: &[u8]) {
    for &b in bytes {
        if *len < LINE_MAX {
            out[*len] = b;
            *len += 1;
        }
    }
}

fn dec(out: &mut [u8; LINE_MAX], len: &mut usize, v: u8) {
    let digits = [v / 100, v / 10 % 10, v % 10];
    let first = digits.iter().position(|&d| d != 0).unwrap_or(2);
    for &d in &digits[first..] {
        push(out, len, &[b'0' + d]);
    }
}

fn addr(out: &mut [u8; LINE_MAX], len: &mut usize, a: &[u8; 4]) {
    if *a == [0; 4] {
        return push(out, len, b"none");
    }
    for (i, &octet) in a.iter().enumerate() {
        if i > 0 {
            push(out, len, b".");
        }
        dec(out, len, octet);
    }
}

/// The line, newline included, and its length.
pub fn lease_line(
    ipv4: &[u8; 4],
    prefix: u8,
    gateway: &[u8; 4],
    server: &[u8; 4],
) -> ([u8; LINE_MAX], usize) {
    let (mut out, mut len) = ([0u8; LINE_MAX], 0);
    push(&mut out, &mut len, b"[DHCP] lease ");
    addr(&mut out, &mut len, ipv4);
    push(&mut out, &mut len, b"/");
    dec(&mut out, &mut len, prefix);
    push(&mut out, &mut len, b" gw ");
    addr(&mut out, &mut len, gateway);
    push(&mut out, &mut len, b" from ");
    addr(&mut out, &mut len, server);
    push(&mut out, &mut len, b"\n");
    (out, len)
}
