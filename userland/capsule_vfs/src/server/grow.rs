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

//! The clients' budget grown to its size for this machine, a piece at a
//! time from the receive loop's idle slot. vfs binds only the floor before
//! it serves (main.rs): backing a quarter of the machine's memory first held
//! its first answer for 15 s on a one-CPU guest with 8 GiB, past every
//! caller's wait. A piece the kernel cannot
//! give, or cannot place at the heap's end, ends the growth where it is,
//! said once.

use nonos_libc::{heap_grow, mk_debug};

use crate::store::budget_size::{limit, next_piece, set};

pub struct Grow {
    wanted: usize,
    done: bool,
}

impl Grow {
    pub fn new(wanted: usize) -> Self {
        Grow { wanted, done: false }
    }

    /// Whether a piece is still to come: the loop polls while it is.
    pub fn pending(&self) -> bool {
        !self.done && next_piece(limit(), self.wanted).is_some()
    }

    /// One piece, if one is due.
    pub fn on_idle(&mut self) {
        if self.done {
            return;
        }
        let Some(piece) = next_piece(limit(), self.wanted) else {
            self.done = true;
            return;
        };
        if heap_grow(piece).is_ok() {
            set(limit() + piece);
            return;
        }
        self.done = true;
        let line = alloc::format!(
            "[VFS] budget held at {} MiB of {} MiB: the heap could grow no further\n",
            limit() >> 20,
            self.wanted >> 20
        );
        let _ = mk_debug(line.as_ptr(), line.len());
    }
}
