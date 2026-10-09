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

//! Bytes out of a family stream.

use alloc::vec::Vec;
use core::mem;

use crate::linux::abi::errno::{EAGAIN, EBADF, ENOTCONN};

use super::table::Socks;

impl Socks {
    /// Up to `want` bytes of a stream; empty is end of file, EAGAIN is none
    /// yet. Bytes come before a pending error, as Linux reads its queue first.
    pub fn read(&mut self, id: u32, want: usize, peek: bool) -> Result<Vec<u8>, i64> {
        let s = self.get_mut(id).ok_or(EBADF)?;
        if !s.rx.is_empty() && want != 0 {
            let n = want.min(s.rx.len());
            return Ok(match peek {
                true => s.rx.iter().take(n).copied().collect(),
                false => s.rx.drain(..n).collect(),
            });
        }
        if s.error != 0 {
            return Err(mem::take(&mut s.error));
        }
        if s.listening || !s.connected {
            return Err(ENOTCONN);
        }
        if want == 0 || s.eof || s.rd_shut {
            return Ok(Vec::new());
        }
        Err(EAGAIN)
    }
}
