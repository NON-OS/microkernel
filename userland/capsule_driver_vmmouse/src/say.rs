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

//! The console lines. They name the backdoor's version and nothing the
//! host or the user did.

use nonos_libc::mk_debug;

const NAME: &[u8] = b"driver.vmmouse: ";

pub fn up(version: u32) {
    let mut d = [0u8; 10];
    line(&[b"up, backdoor version ", dec(version, &mut d), b", absolute pointer on"]);
}

pub fn lost() {
    line(&[b"pointer lost, enabling it again"]);
}

pub fn back() {
    line(&[b"pointer back"]);
}

fn line(parts: &[&[u8]]) {
    let mut out = [0u8; 96];
    let mut n = 0;
    for &b in NAME.iter().chain(parts.iter().flat_map(|p| p.iter())) {
        if n == out.len() - 1 {
            break;
        }
        out[n] = b;
        n += 1;
    }
    out[n] = b'\n';
    let _ = mk_debug(out.as_ptr(), n + 1);
}

fn dec(mut v: u32, buf: &mut [u8; 10]) -> &[u8] {
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 {
            return &buf[i..];
        }
    }
}
