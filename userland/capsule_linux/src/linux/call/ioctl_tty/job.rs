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

//! The terminal requests about its settings and its process groups; the
//! window size, the queues and flow control are size.rs's.

use crate::linux::abi::errno;
use crate::linux::console;
use crate::linux::console::termios2::{from2, to2, SIZE2};
use crate::linux::guest::Guest;
use crate::linux::serve::guest_pid;

use super::super::ioctl_req::Req;
use super::size::{put, size};

/// The answer to terminal request `req`, its argument `arg`.
pub(in super::super) fn tty(guest: &mut Guest, req: Req, arg: u64) -> u64 {
    match req {
        Req::GetTermios => put(guest, arg, &console::termios()),
        Req::SetTermios { flush } => match guest.read(arg, console::termios().len()) {
            Some(raw) => set(&raw, flush),
            None => errno::fail(errno::EFAULT),
        },
        Req::GetTermios2 => put(guest, arg, &to2(&console::termios())),
        Req::SetTermios2 { flush } => match guest.read(arg, SIZE2) {
            Some(raw) => match from2(&raw, &console::termios()) {
                Some(t) => set(&t, flush),
                None => errno::fail(errno::EFAULT),
            },
            None => errno::fail(errno::EFAULT),
        },
        /*
         * The caller's own group and session, in the family's numbering as
         * getpgrp and getsid give them; the foreground group is the one the
         * shell last set with TIOCSPGRP, else the caller's own.
         */
        Req::Pgrp => {
            let fg = console::fg().unwrap_or_else(|| guest_pid(guest.pgid));
            put(guest, arg, &fg.to_le_bytes())
        }
        Req::Sid => put(guest, arg, &guest_pid(guest.sid).to_le_bytes()),
        /* A group must be a real one, as Linux's tiocspgrp refuses below zero. */
        Req::SetPgrp => match guest.read(arg, 4) {
            Some(raw) if i32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]) < 0 => {
                errno::fail(errno::EINVAL)
            }
            Some(raw) => {
                console::set_fg(u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]));
                errno::ok(0)
            }
            None => errno::fail(errno::EFAULT),
        },
        other => size(guest, other, arg),
    }
}

/// The settings `raw` taken, and typed input dropped first when `flush`.
fn set(raw: &[u8], flush: bool) -> u64 {
    console::set_termios(raw);
    if flush {
        console::flush();
    }
    errno::ok(0)
}
