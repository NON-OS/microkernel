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

/* What each descriptor names, and fdinfo's lines. */

use alloc::vec::Vec;

use crate::linux::file::flags::{O_CLOEXEC, O_NONBLOCK, O_RDWR, O_WRONLY};
use crate::linux::guest::{Fd, Kind};

use super::super::super::view::Proc;
use super::list::{CONSOLE_IN, CONSOLE_OUT, PIPES, SOCKETS};

pub fn fd_target(f: &Fd) -> Vec<u8> {
    match f.kind {
        Kind::File | Kind::Dir | Kind::Device => f.path.clone(),
        Kind::Stdin => alloc::format!("pipe:[{CONSOLE_IN}]").into_bytes(),
        Kind::Stdout | Kind::Stderr => alloc::format!("pipe:[{CONSOLE_OUT}]").into_bytes(),
        Kind::Pipe => alloc::format!("pipe:[{}]", PIPES + u64::from(f.handle)).into_bytes(),
        Kind::Socket | Kind::Unix | Kind::Resolver => {
            alloc::format!("socket:[{}]", SOCKETS + u64::from(f.handle)).into_bytes()
        }
        Kind::Memfd => b"/memfd: (deleted)".to_vec(),
        Kind::Epoll => b"anon_inode:[eventpoll]".to_vec(),
        Kind::Timer => b"anon_inode:[timerfd]".to_vec(),
        Kind::Event => b"anon_inode:[eventfd]".to_vec(),
        Kind::Signal => b"anon_inode:[signalfd]".to_vec(),
        Kind::Free => Vec::new(),
    }
}

pub(super) fn flags_of(f: &Fd) -> u64 {
    let mode = match (f.kind, f.writable) {
        (Kind::Stdout | Kind::Stderr, _) => O_WRONLY,
        (Kind::Stdin | Kind::Dir, _) => 0,
        (Kind::Pipe | Kind::File, true) => O_WRONLY,
        (Kind::Pipe | Kind::File, false) => 0,
        _ => O_RDWR,
    };
    let nonblock = if f.nonblock { O_NONBLOCK } else { 0 };
    mode | nonblock | if f.cloexec { O_CLOEXEC } else { 0 }
}

pub fn target_of(proc: &Proc, fd: u32) -> Option<Vec<u8>> {
    proc.fds.iter().find(|f| f.fd == fd).map(|f| f.target.clone())
}

/*
 * fdinfo: the position and the flags, in octal as Linux prints them, and
 * the mount a file is on. Nothing else is claimed.
 */
pub fn info(proc: &Proc, fd: u32) -> Option<Vec<u8>> {
    let f = proc.fds.iter().find(|f| f.fd == fd)?;
    let mut out = alloc::format!("pos:\t{}\nflags:\t0{:o}\n", f.offset, f.flags);
    if f.target.first() == Some(&b'/') {
        out.push_str(&alloc::format!("mnt_id:\t{}\n", super::super::mounts::of(&f.target).0));
    }
    Some(out.into_bytes())
}
