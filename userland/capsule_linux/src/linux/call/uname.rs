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

/*
 * `uname`. Six fixed fields of sixty-five bytes, in Linux's order.
 *
 * `sysname` says Linux because it names the ABI this capsule implements,
 * which is the question the caller is asking: a program reads it to
 * decide which syscalls exist. What machine it is really running on is
 * in the other fields, and they say NONOS rather than pretending.
 */

use crate::linux::abi::errno;
use crate::linux::file::declared;
use crate::linux::guest::Guest;

const FIELD: usize = 65;
const UTSNAME_LEN: usize = FIELD * 6;

/*
 * The oldest release that has every call this capsule serves. A program
 * gating a feature on the version gets an answer that matches what it
 * will actually find here.
 */
pub fn uname(guest: &mut Guest, out: u64) -> u64 {
    let mut buf = [0u8; UTSNAME_LEN];
    put(&mut buf, 0, declared::OSTYPE);
    put(&mut buf, 1, declared::HOSTNAME);
    put(&mut buf, 2, declared::RELEASE);
    put(&mut buf, 3, declared::VERSION);
    put(&mut buf, 4, declared::MACHINE);
    put(&mut buf, 5, declared::DOMAIN);
    if guest.write(out, &buf) < UTSNAME_LEN as i64 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}

fn put(buf: &mut [u8; UTSNAME_LEN], field: usize, value: &[u8]) {
    let at = field * FIELD;
    let n = value.len().min(FIELD - 1);
    buf[at..at + n].copy_from_slice(&value[..n]);
}
