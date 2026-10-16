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

//! The `net0` line read out of a DHCP client's lease reply.

use alloc::vec::Vec;

use super::HDR_LEN;
use crate::term::util::format_u64;

/// Address, prefix, gateway and resolver from the reply in `rx`.
pub(super) fn lease_line(rx: &[u8]) -> Vec<u8> {
    let mut line: Vec<u8> = Vec::new();
    line.extend_from_slice(b"net0: inet ");
    push_ipv4(&mut line, &rx[HDR_LEN + 1..HDR_LEN + 5]);
    line.push(b'/');
    push_num(&mut line, rx[HDR_LEN + 5] as u64);
    line.extend_from_slice(b" gw ");
    push_ipv4(&mut line, &rx[HDR_LEN + 6..HDR_LEN + 10]);
    line.extend_from_slice(b" dns ");
    push_ipv4(&mut line, &rx[HDR_LEN + 10..HDR_LEN + 14]);
    line
}

fn push_ipv4(out: &mut Vec<u8>, ip: &[u8]) {
    for (i, &b) in ip.iter().enumerate() {
        if i > 0 {
            out.push(b'.');
        }
        push_num(out, b as u64);
    }
}

fn push_num(out: &mut Vec<u8>, v: u64) {
    let mut buf = [0u8; 24];
    let k = format_u64(v, &mut buf);
    out.extend_from_slice(&buf[..k]);
}
