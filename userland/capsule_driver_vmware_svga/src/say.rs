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

//! The boot line: the mode the screen runs in, and whether the firmware
//! chose it.

use nonos_libc::mk_debug;

use crate::display::Svga;

pub fn up(d: &Svga) {
    let mut line = [0u8; 64];
    let mut n = 0;
    let mut push = |bytes: &[u8]| {
        for &b in bytes.iter().take(line.len() - 1 - n) {
            line[n] = b;
            n += 1;
        }
    };
    push(b"driver.vmware_svga: up, ");
    push(dec(d.mode.width).as_slice());
    push(b"x");
    push(dec(d.mode.height).as_slice());
    push(b"x32");
    line[n] = b'\n';
    let _ = mk_debug(line.as_ptr(), n + 1);
}

fn dec(v: u32) -> alloc::vec::Vec<u8> {
    let mut out = alloc::vec::Vec::new();
    let mut n = v;
    loop {
        out.insert(0, b'0' + (n % 10) as u8);
        n /= 10;
        if n == 0 {
            return out;
        }
    }
}
