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

//! One log line about a root port, tagged so `log usb` finds it.

const PREFIX: &[u8] = b"[usb-hid] port ";

pub(super) fn say_port(port: u8, what: &[u8]) {
    let mut line = [0u8; 160];
    let mut n = 0;
    let digits = decimal(port);
    let parts = [PREFIX, &digits.0[..digits.1], b": ", what, b"\n"];
    for &b in parts.iter().flat_map(|p| p.iter()) {
        if n == line.len() - 1 {
            break;
        }
        line[n] = b;
        n += 1;
    }
    if line[n.saturating_sub(1)] != b'\n' {
        line[n] = b'\n';
        n += 1;
    }
    let _ = nonos_libc::mk_debug(line.as_ptr(), n);
}

/// `v` in decimal without leading zeros, and how many digits that is.
fn decimal(v: u8) -> ([u8; 3], usize) {
    let all = [b'0' + v / 100, b'0' + (v / 10) % 10, b'0' + v % 10];
    let skip = if v >= 100 {
        0
    } else if v >= 10 {
        1
    } else {
        2
    };
    let mut out = [0u8; 3];
    out[..3 - skip].copy_from_slice(&all[skip..]);
    (out, 3 - skip)
}
