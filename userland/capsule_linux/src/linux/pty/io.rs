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

//! A pseudo-terminal descriptor read and written, through the family's
//! table. A signal typed at the master is left for the serve loop
//! to deliver to the foreground group (serve::family_pty).

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::cell::with;
use super::fd::side;

/// The most one read or write moves; a caller asks again for the rest.
const STEP: u64 = 64 << 10;

pub fn read(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some((n, master)) = side(guest, fd) else {
        return errno::fail(errno::EBADF);
    };
    if len == 0 {
        return errno::ok(0);
    }
    let most = len.min(STEP) as usize;
    let got =
        with(|t| t.get(n).map(|p| if master { p.master_read(most) } else { p.slave_read(most) }));
    match got.unwrap_or(Err(errno::EIO)) {
        Ok(bytes) if guest.write(buf, &bytes) == bytes.len() as i64 => {
            errno::ok(bytes.len() as u64)
        }
        Ok(_) => errno::fail(errno::EFAULT),
        Err(e) => errno::fail(e),
    }
}

pub fn write(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some((n, master)) = side(guest, fd) else {
        return errno::fail(errno::EBADF);
    };
    let Some(bytes) = guest.read(buf, len.min(STEP) as usize) else {
        return errno::fail(errno::EFAULT);
    };
    let got = with(|t| match master {
        true => t.typed_at(n, &bytes),
        false => t.get(n).ok_or(errno::EIO)?.slave_write(&bytes),
    });
    match got {
        Ok(n) => errno::ok(n as u64),
        Err(e) => errno::fail(e),
    }
}
