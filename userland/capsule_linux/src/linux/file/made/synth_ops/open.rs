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

/* Opening a path in a made tree. */

use alloc::string::String;

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::super::super::flags::{O_CREAT, O_DIRECTORY};
use super::super::super::slot;
use super::super::dev;
use super::super::synth::{self, Node};
use super::made::{made_file, refuse};

/*
 * Open `path`, already followed, if it is one of these files; None if it
 * is not in /dev, /proc or /sys.
 */
pub fn open(guest: &mut Guest, path: &[u8], flags: u64) -> Option<u64> {
    let node = match synth::node(path)? {
        Ok(node) => node,
        /* Those trees are mounted read-only: nothing is made in them. */
        Err(_) if flags & O_CREAT != 0 => return Some(errno::fail(errno::EROFS)),
        Err(e) => return Some(errno::fail(e)),
    };
    /* O_WRONLY or O_RDWR. */
    let writing = flags & 3 != 0;
    let fd = match node {
        Node::Dir(_) if writing => return Some(errno::fail(errno::EISDIR)),
        Node::Dir(names) => {
            let dots = [String::from("."), String::from("..")];
            let names =
                dots.into_iter().chain(names.into_iter().filter_map(|n| String::from_utf8(n).ok()));
            let mut fd = Fd::dir(path.to_vec(), names.collect());
            fd.handle = super::super::super::desc::fresh(false, false);
            fd
        }
        _ if flags & O_DIRECTORY != 0 => return Some(errno::fail(errno::ENOTDIR)),
        Node::Dev(dev::Dev::Tty) => return Some(errno::fail(errno::ENXIO)),
        Node::Dev(_) => made_file(path, writing, flags),
        Node::Text(_) if writing => return Some(errno::fail(errno::EACCES)),
        Node::Text(_) => made_file(path, false, flags),
        Node::Refused(why) => return Some(refuse(why)),
        Node::Link(to) => return Some(super::super::fdopen::reopen(guest, path, &to, flags)),
    };
    Some(match slot::install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    })
}
