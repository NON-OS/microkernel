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

//! What a pipe end can do now, in poll's bits, as Linux's `pipe_poll` says
//! it: a read end is readable while it holds bytes and hung up once no write
//! end is left; a write end is writable while there is room and in error
//! once no read end is left.

use crate::linux::guest::Guest;
use crate::linux::net::{POLLERR, POLLHUP};

use super::pipe_end::{end_of, other_end_open};
use super::pipe_io::CAPACITY;

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLNVAL: u16 = 0x020;

pub fn bits(guest: &Guest, fd: u64) -> u16 {
    let Some((slot, writable)) = end_of(guest, fd) else {
        return POLLNVAL;
    };
    let held = guest.pipes[slot].len();
    let other = other_end_open(guest, slot, writable);
    match writable {
        false => flag(held > 0, POLLIN) | flag(!other, POLLHUP),
        true => flag(held < CAPACITY, POLLOUT) | flag(!other, POLLERR),
    }
}

fn flag(on: bool, bit: u16) -> u16 {
    if on {
        bit
    } else {
        0
    }
}
