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

//! Reading and writing the character devices, as Linux's drivers answer:
//! /dev/null reads end of file and takes every write, /dev/zero reads zeros
//! and takes every write, /dev/full reads zeros and refuses every write with
//! ENOSPC, and the two random devices read the kernel's random bytes and take
//! writes without keeping them, as an unprivileged write to them does.

use alloc::vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, MAX_SPAN};

use super::dev::{FULL, NULL, ZERO};

/// Linux's random devices hand at most this much to one read.
const RANDOM_MAX: u64 = 32 << 20;

fn device(guest: &Guest, fd: u64) -> u32 {
    guest.fds.get(fd as usize).map_or(u32::MAX, |f| f.handle)
}

pub fn read(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let dev = device(guest, fd);
    if dev == NULL {
        return errno::ok(0);
    }
    /* One step at a time, so a large read never holds a large buffer. */
    let want = if dev == ZERO || dev == FULL { len } else { len.min(RANDOM_MAX) };
    let mut done = 0;
    while done < want {
        let take = (want - done).min(MAX_SPAN) as usize;
        let mut bytes = vec![0u8; take];
        if dev != ZERO && dev != FULL && nonos_libc::crypto_random(bytes.as_mut_ptr(), take) < 0 {
            break;
        }
        if guest.write(buf + done, &bytes) < take as i64 {
            return if done == 0 { errno::fail(errno::EFAULT) } else { errno::ok(done) };
        }
        done += take as u64;
    }
    errno::ok(done)
}

pub fn write(guest: &mut Guest, fd: u64, len: u64) -> u64 {
    match device(guest, fd) {
        FULL => errno::fail(errno::ENOSPC),
        _ => errno::ok(len),
    }
}
