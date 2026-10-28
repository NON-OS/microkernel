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

//! The terminal requests on a pseudo-terminal descriptor: its settings,
//! window size and queues here, its sessions and groups in ioctl_job.rs.
//! Both sides act on the one set of settings, as Linux's tty_mode_ioctl
//! has a master's requests act on its slave.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::cell::with;
use super::fd::{queued, side};
use super::ioctl_set::{flush, set_termios, set_winsize};

const TCGETS: u64 = 0x5401;
const TCSETS: u64 = 0x5402;
const TCSETSW: u64 = 0x5403;
const TCSETSF: u64 = 0x5404;
const TCFLSH: u64 = 0x540B;
const TIOCGWINSZ: u64 = 0x5413;
const TIOCSWINSZ: u64 = 0x5414;
const FIONREAD: u64 = 0x541B;

/// The answer to `req` on `fd`, or None when `fd` is no pseudo-terminal or
/// the request is a file's (FIOCLEX, FIONBIO), which ioctl answers itself.
pub fn ioctl(guest: &mut Guest, fd: u64, req: u64, arg: u64) -> Option<u64> {
    let (n, master) = side(guest, fd)?;
    let at = |t: &mut super::table::Table| t.get(n).map(|p| (p.termios, p.winsize));
    let (termios, winsize) = with(at)?;
    let r = req & 0xFFFF_FFFF;
    Some(match r {
        TCGETS => put(guest, arg, &termios),
        TCSETS | TCSETSW | TCSETSF => set_termios(guest, n, r == TCSETSF, arg),
        TIOCGWINSZ => put(guest, arg, &winsize),
        TIOCSWINSZ => set_winsize(guest, n, arg),
        FIONREAD => put(guest, arg, &(queued(guest, fd)? as i32).to_le_bytes()),
        TCFLSH => flush(n, master, arg),
        _ => return super::ioctl_job::ioctl(guest, n, master, r, arg),
    })
}

/// `bytes` written at `arg`: zero, or EFAULT.
pub(super) fn put(guest: &Guest, arg: u64, bytes: &[u8]) -> u64 {
    match guest.write(arg, bytes) == bytes.len() as i64 {
        true => errno::ok(0),
        false => errno::fail(errno::EFAULT),
    }
}
