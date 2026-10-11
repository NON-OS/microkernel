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

//! A read from a net.anon stream's buffer.

use alloc::vec::Vec;

use crate::linux::abi::errno::EAGAIN;

use super::super::backend::{Anon, End};

impl Anon {
    /// What a read of `want` bytes finds: held bytes, end of file once the
    /// read side is shut or the far end finished, the far end's error once,
    /// or EAGAIN while the stream is open and nothing has come.
    pub fn take(&mut self, want: usize, peek: bool, rd_shut: bool) -> Result<Vec<u8>, i64> {
        if want == 0 || rd_shut {
            return Ok(Vec::new());
        }
        if !self.rx.is_empty() {
            let n = want.min(self.rx.len());
            let out = self.rx[..n].to_vec();
            if !peek {
                self.rx.drain(..n);
            }
            return Ok(out);
        }
        match self.end {
            /* Reported once, as Linux reports a socket's error. */
            Some(End::Error(e)) => {
                self.end = Some(End::Eof);
                Err(e)
            }
            Some(End::Eof) => Ok(Vec::new()),
            None if self.id.is_none() => Ok(Vec::new()),
            None => Err(EAGAIN),
        }
    }
}
