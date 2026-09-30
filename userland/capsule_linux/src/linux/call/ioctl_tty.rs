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
//! on a terminal, so a program knows it is on one and line-buffers its
//! output. On no terminal they are not answered here, and stay ENOTTY.

use crate::linux::abi::errno;
use crate::linux::console;
use crate::linux::guest::{Guest, Kind};

const TCGETS: u64 = 0x5401;
const TCSETS: u64 = 0x5402;
const TCSETSW: u64 = 0x5403;
const TCSETSF: u64 = 0x5404;
const TIOCGPGRP: u64 = 0x540F;
const TIOCGWINSZ: u64 = 0x5413;
const FIONREAD: u64 = 0x541B;

/// The answer to a terminal request on `fd`, or None to leave it to ioctl.
pub(super) fn tty(guest: &Guest, fd: u64, request: u64, arg: u64) -> Option<u64> {
    let kind = guest.fds.get(fd as usize)?.kind;
    if !matches!(kind, Kind::Stdin | Kind::Stdout | Kind::Stderr) {
        return None;
    }
    let request = request & 0xFFFF_FFFF;
    let known = matches!(request, TCGETS | TCSETS | TCSETSW | TCSETSF | TIOCGPGRP | TIOCGWINSZ);
    if !(known || (request == FIONREAD && kind == Kind::Stdin)) || !console::attached() {
        return None;
    }
    let put = |bytes: &[u8]| match guest.write(arg, bytes) == bytes.len() as i64 {
        true => errno::ok(0),
        false => errno::fail(errno::EFAULT),
    };
    Some(match request {
        TCGETS => put(&console::termios()),
        TCSETS | TCSETSW | TCSETSF => match guest.read(arg, console::termios().len()) {
            Some(raw) => {
                console::set_termios(&raw);
                if request == TCSETSF {
                    console::flush();
                }
                errno::ok(0)
            }
            None => errno::fail(errno::EFAULT),
        },
        /*
         * The caller's own group: whoever asks is in the foreground.
         */
        TIOCGPGRP => put(&guest.pgid.to_le_bytes()),
        TIOCGWINSZ => {
            let (rows, cols) = console::size();
            let mut ws = [0u8; 8];
            ws[..2].copy_from_slice(&rows.to_le_bytes());
            ws[2..4].copy_from_slice(&cols.to_le_bytes());
            put(&ws)
        }
        _ => put(&(console::queued().min(i32::MAX as u64) as i32).to_le_bytes()),
    })
}
