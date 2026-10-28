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

//! What a read or a write on either side of a pair answers, as Linux's pty
//! and n_tty do: EAGAIN (which parks a blocking caller) while it would wait,
//! end of file on a slave once its master has gone, and EIO on a master
//! once every slave has closed. Pure.

use alloc::vec::Vec;

use super::flags::VMIN;
use super::input::Taken;
use super::pair::Pair;
use crate::linux::abi::errno::{EAGAIN, EIO};

impl Pair {
    /// A master's read: what the slave wrote and the echo, in order.
    pub fn master_read(&mut self, most: usize) -> Result<Vec<u8>, i64> {
        let n = most.min(self.output.len());
        match n {
            0 if self.slave_seen && !self.slave => Err(EIO),
            0 => Err(EAGAIN),
            _ => Ok(self.output.drain(..n).collect()),
        }
    }

    /// A slave's read; empty is end of file. Raw with VMIN 0 never waits.
    pub fn slave_read(&mut self, most: usize) -> Result<Vec<u8>, i64> {
        if self.hung {
            return Ok(Vec::new());
        }
        match self.input.take(most, self.canon()) {
            Taken::Bytes(bytes) => Ok(bytes),
            Taken::Eof => Ok(Vec::new()),
            Taken::Empty if !self.canon() && self.termios[17 + VMIN] == 0 => Ok(Vec::new()),
            Taken::Empty => Err(EAGAIN),
        }
    }

    /// A master's write: how much went in, and the signals it raised.
    pub fn master_write(&mut self, bytes: &[u8]) -> Result<(usize, Vec<u8>), i64> {
        if self.slave_seen && !self.slave {
            return Err(EIO);
        }
        match self.typed(bytes) {
            (0, _) if !bytes.is_empty() => Err(EAGAIN),
            got => Ok(got),
        }
    }

    /// A slave's write: EIO once hung up.
    pub fn slave_write(&mut self, bytes: &[u8]) -> Result<usize, i64> {
        if self.hung {
            return Err(EIO);
        }
        match self.written(bytes) {
            0 if !bytes.is_empty() => Err(EAGAIN),
            n => Ok(n),
        }
    }
}
