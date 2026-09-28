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

//! `ioctl`: the requests that are about a descriptor rather than a device.
//! There is no terminal or device behind any descriptor here, so anything
//! else is ENOTTY, which is also how a program learns it is not on a tty.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

const FIONREAD: u64 = 0x541B;
const FIONBIO: u64 = 0x5421;
const FIONCLEX: u64 = 0x5450;
const FIOCLEX: u64 = 0x5451;

pub fn ioctl(guest: &mut Guest, fd: u64, request: u64, arg: u64) -> u64 {
    let Some(entry) = guest.fds.get_mut(fd as usize).filter(|e| e.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    match request & 0xFFFF_FFFF {
        FIOCLEX | FIONCLEX => {
            entry.cloexec = request & 0xFFFF_FFFF == FIOCLEX;
            errno::ok(0)
        }
        FIONBIO => match guest.read(arg, 4) {
            Some(raw) => {
                let on = raw.iter().any(|&b| b != 0);
                if let Some(entry) = guest.fds.get_mut(fd as usize) {
                    entry.nonblock = on;
                }
                errno::ok(0)
            }
            None => errno::fail(errno::EFAULT),
        },
        FIONREAD => match waiting(guest, fd) {
            Some(n) if guest.write(arg, &(n.min(i32::MAX as u64) as i32).to_le_bytes()) == 4 => {
                errno::ok(0)
            }
            Some(_) => errno::fail(errno::EFAULT),
            None => errno::fail(errno::ENOTTY),
        },
        _ => errno::fail(errno::ENOTTY),
    }
}

/// Bytes a read would find now: what a pipe holds, or what is left of a
/// file past its offset. None for a descriptor Linux answers ENOTTY for.
fn waiting(guest: &Guest, fd: u64) -> Option<u64> {
    let entry = guest.fds.get(fd as usize)?;
    match entry.kind {
        Kind::Pipe if !entry.writable => {
            guest.pipes.get(entry.handle as usize).map(|p| p.len() as u64)
        }
        Kind::File => Some(entry.size.saturating_sub(entry.offset)),
        _ => None,
    }
}
