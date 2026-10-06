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

/* A process's descriptors as /proc/<pid>/fd lists them. */

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::super::view::Open;
use super::info::{fd_target, flags_of};

/*
 * Inode numbers for what has no file: the console's two streams, then
 * each pipe by its buffer, then each socket by its handle.
 */
pub const CONSOLE_IN: u64 = 1;

pub const CONSOLE_OUT: u64 = 2;

pub const PIPES: u64 = 0x1000;

pub const SOCKETS: u64 = 0x10_0000;

pub fn open_fds(guest: &Guest) -> Vec<Open> {
    let open = guest.fds.iter().enumerate().filter(|(_, f)| f.is_open());
    open.map(|(i, f)| Open {
        fd: i as u32,
        target: fd_target(f),
        offset: super::super::super::super::desc::pos(f),
        flags: flags_of(f),
        desc: super::super::super::super::desc::of(f),
    })
    .collect()
}
