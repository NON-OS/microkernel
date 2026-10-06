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
//! Which requests a descriptor answers is `ioctl_req`, which the host proofs
//! hold; a console on a terminal answers the terminal's own (`ioctl_tty`);
//! anything else is ENOTTY, which is how a program learns it is not on one.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::ioctl_req::{request, Req};

pub fn ioctl(guest: &mut Guest, fd: u64, req: u64, arg: u64) -> u64 {
    let Some(kind) = guest.fds.get(fd as usize).filter(|e| e.is_open()).map(|e| e.kind) else {
        return errno::fail(errno::EBADF);
    };
    let console = matches!(kind, Kind::Stdin | Kind::Stdout | Kind::Stderr);
    let on_terminal = console && crate::linux::console::on_tty(stream(kind));
    match request(kind, on_terminal, req) {
        Req::CloseOnExec(on) => {
            if let Some(entry) = guest.fds.get_mut(fd as usize) {
                entry.cloexec = on;
            }
            errno::ok(0)
        }
        Req::NonBlock => match guest.read(arg, 4) {
            Some(raw) => {
                let on = raw.iter().any(|&b| b != 0);
                if let Some(entry) = guest.fds.get_mut(fd as usize) {
                    entry.nonblock = on;
                }
                errno::ok(0)
            }
            None => errno::fail(errno::EFAULT),
        },
        Req::Queued => match waiting(guest, fd, on_terminal) {
            Some(n) if guest.write(arg, &(n.min(i32::MAX as u64) as i32).to_le_bytes()) == 4 => {
                errno::ok(0)
            }
            Some(_) => errno::fail(errno::EFAULT),
            None => errno::fail(errno::ENOTTY),
        },
        Req::NotTty => errno::fail(errno::ENOTTY),
        tty => super::ioctl_tty::tty(guest, tty, arg),
    }
}

/// Bytes a read would find now: what a pipe holds, from either end, as
/// Linux's pipe_ioctl counts it; what is left of a file past its offset;
/// what was typed at the terminal; and nothing for the console off one,
/// which is a pipe at its end. None for a descriptor Linux answers ENOTTY.
fn waiting(guest: &Guest, fd: u64, on_terminal: bool) -> Option<u64> {
    let entry = guest.fds.get(fd as usize)?;
    match entry.kind {
        _ if on_terminal => Some(crate::linux::console::queued()),
        Kind::Stdin => Some(0),
        Kind::Pipe => guest.pipes.get(entry.handle as usize).map(|p| p.len() as u64),
        Kind::File => Some(entry.size.saturating_sub(entry.offset)),
        _ => None,
    }
}

/// The standard stream a console descriptor is: 0, 1 or 2.
pub(crate) fn stream(kind: Kind) -> u32 {
    match kind {
        Kind::Stdin => 0,
        Kind::Stdout => 1,
        _ => 2,
    }
}
