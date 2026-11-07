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

//! `mem`: the memory the machine's processes hold, of its total, in KiB.

use alloc::vec::Vec;

use crate::host::Host;
use crate::out::ShellOut;
use crate::util::format_u64;

pub fn run(out: &mut dyn ShellOut, host: &mut dyn Host) -> bool {
    let Some(m) = host.mem() else {
        out.writeln(b"mem: unavailable");
        return false;
    };
    let mut line = b"mem: ".to_vec();
    kib(&mut line, m.used);
    line.extend_from_slice(b" KiB used of ");
    kib(&mut line, m.total);
    line.extend_from_slice(b" KiB");
    if let Some(pct) = m.used.checked_mul(100).and_then(|u| u.checked_div(m.total)) {
        line.extend_from_slice(b" (");
        push(&mut line, pct);
        line.extend_from_slice(b"%)");
    }
    out.writeln(&line);
    true
}

fn kib(out: &mut Vec<u8>, bytes: u64) {
    push(out, bytes / 1024);
}

fn push(out: &mut Vec<u8>, v: u64) {
    let mut buf = [0u8; 20];
    let n = format_u64(v, &mut buf);
    out.extend_from_slice(&buf[..n]);
}
