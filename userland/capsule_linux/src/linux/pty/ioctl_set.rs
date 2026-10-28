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

//! The requests that change a pair: new settings, a new window size, and
//! what is queued dropped.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::cell::with;
use super::flags::Termios;
use super::ioctl_ctty::on;

/// TCSETS and TCSETSW, and TCSETSF, which drops what was typed first.
pub(super) fn set_termios(guest: &Guest, n: u32, flush: bool, arg: u64) -> u64 {
    let Some(t) = guest.read(arg, 36).and_then(|raw| Termios::try_from(raw).ok()) else {
        return errno::fail(errno::EFAULT);
    };
    on(n, |p| {
        if flush {
            p.input = Default::default();
        }
        p.set_termios(t);
    })
}

/// TIOCSWINSZ: a size that changed raises SIGWINCH for the foreground group.
pub(super) fn set_winsize(guest: &Guest, n: u32, arg: u64) -> u64 {
    let Some(ws) = guest.read(arg, 8).and_then(|raw| <[u8; 8]>::try_from(raw).ok()) else {
        return errno::fail(errno::EFAULT);
    };
    with(|t| t.resize(n, ws)).map_or_else(errno::fail, |()| errno::ok(0))
}

/// TCFLSH: TCIFLUSH drops what this side would read, TCOFLUSH what it
/// wrote and the other side has not read, TCIOFLUSH both.
pub(super) fn flush(n: u32, master: bool, arg: u64) -> u64 {
    let (read, written) = match arg as u32 {
        0 => (true, false),
        1 => (false, true),
        2 => (true, true),
        _ => return errno::fail(errno::EINVAL),
    };
    let (typed, output) = if master { (written, read) } else { (read, written) };
    on(n, |p| {
        if typed {
            p.input = Default::default();
        }
        if output {
            p.output.clear();
        }
    })
}
