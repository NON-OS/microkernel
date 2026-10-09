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

//! The terminal requests a console descriptor answers when the family is
//! on a terminal (`ioctl_req` decides which), so a program knows it is on
//! one and line-buffers its output.

use crate::linux::abi::errno;
use crate::linux::console;
use crate::linux::guest::siginfo::{SigInfo, SI_KERNEL};
use crate::linux::guest::Guest;
use crate::linux::serve::guest_pid;

use super::ioctl_req::{flow_ok, flush_input, winsize, Req};

/// SIGWINCH, raised when TIOCSWINSZ changes the size.
const SIGWINCH: u8 = 28;

/// The answer to terminal request `req`, its argument `arg`.
pub(super) fn tty(guest: &mut Guest, req: Req, arg: u64) -> u64 {
    let put = |guest: &Guest, bytes: &[u8]| match guest.write(arg, bytes) == bytes.len() as i64 {
        true => errno::ok(0),
        false => errno::fail(errno::EFAULT),
    };
    match req {
        Req::GetTermios => put(guest, &console::termios()),
        Req::SetTermios { flush } => match guest.read(arg, console::termios().len()) {
            Some(raw) => {
                console::set_termios(&raw);
                if flush {
                    console::flush();
                }
                errno::ok(0)
            }
            None => errno::fail(errno::EFAULT),
        },
        /*
         * The caller's own group and session, in the family's numbering as
         * getpgrp and getsid give them; the foreground group is the one the
         * shell last set with TIOCSPGRP, else the caller's own.
         */
        Req::Pgrp => {
            let fg = console::fg().unwrap_or_else(|| guest_pid(guest.pgid));
            put(guest, &fg.to_le_bytes())
        }
        Req::Sid => put(guest, &guest_pid(guest.sid).to_le_bytes()),
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
        Req::GetWinsize => {
            let (rows, cols) = console::size();
            let mut ws = [0u8; 8];
            ws[..2].copy_from_slice(&rows.to_le_bytes());
            ws[2..4].copy_from_slice(&cols.to_le_bytes());
            put(guest, &ws)
        }
        Req::SetWinsize => match guest.read(arg, 8).as_deref().and_then(winsize) {
            Some((rows, cols)) => {
                if console::set_size(rows, cols) {
                    let _ = guest.raise_and_wake(0, SigInfo::from(SIGWINCH, SI_KERNEL, 0));
                }
                errno::ok(0)
            }
            None => errno::fail(errno::EFAULT),
        },
        Req::OutQueued => put(guest, &0i32.to_le_bytes()),
        Req::Flush => match flush_input(arg) {
            Ok(input) => {
                if input {
                    console::flush();
                }
                errno::ok(0)
            }
            Err(e) => errno::fail(e),
        },
        Req::Drain => errno::ok(0),
        Req::FlowControl => match flow_ok(arg) {
            Ok(()) => errno::ok(0),
            Err(e) => errno::fail(e),
        },
        _ => errno::fail(errno::ENOTTY),
    }
}
