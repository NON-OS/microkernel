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

//! What a guest can learn that would tell this machine from another.
//!
//! Each value must be the one every install gives: a constant name, a pid in
//! the family's own numbering, the same volume size, and randomness that is
//! fresh on every read. Anything else is a fingerprint, and it is reported.

use crate::report::{Report, Seen};
use crate::sys::{call, GETPID};

const UNAME: u64 = 63;
const STATFS: u64 = 137;
const GETPPID: u64 = 110;
const GETRANDOM: u64 = 318;

pub fn scan(r: &mut Report) {
    let mut uts = [0u8; 390];
    let _ = call(UNAME, [uts.as_mut_ptr() as u64, 0, 0, 0, 0, 0]);
    let node = field(&uts, 1);
    r.check("the host name", same(node == b"nonos", || format!("nodename {:?}", node)));
    let (pid, ppid) = (call(GETPID, [0; 6]), call(GETPPID, [0; 6]));
    let numbered = pid == 2 && ppid == 1;
    r.check("the machine's pid count", same(numbered, || format!("pid {pid}, parent {ppid}")));
    let mut fs = [0u64; 15];
    let _ = call(STATFS, [b"/\0".as_ptr() as u64, fs.as_mut_ptr() as u64, 0, 0, 0, 0]);
    let plain = fs[2] == 1 << 20 && fs[3] == 1 << 19;
    r.check("the store's size", same(plain, || format!("{} blocks, {} free", fs[2], fs[3])));
    let (a, b) = (random(), random());
    r.check("repeated randomness", same(a != b && a != [0; 16], || "two reads agreed".into()));
}

fn same(held: bool, how: impl FnOnce() -> String) -> Seen {
    match held {
        true => Seen::Refused(0),
        false => Seen::Escaped(how()),
    }
}

fn field(uts: &[u8; 390], i: usize) -> &[u8] {
    let f = &uts[i * 65..(i + 1) * 65];
    &f[..f.iter().position(|b| *b == 0).unwrap_or(65)]
}

fn random() -> [u8; 16] {
    let mut buf = [0u8; 16];
    let _ = call(GETRANDOM, [buf.as_mut_ptr() as u64, 16, 0, 0, 0, 0]);
    buf
}
