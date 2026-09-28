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

//! A datagram out of the family's queue for one socket.

use alloc::vec::Vec;
use core::mem;

use crate::linux::abi::errno::{EAGAIN, EBADF};

use super::table::Socks;
use super::types::Addr;

impl Socks {
    /// The next datagram, cut to `want`, with its whole length and sender.
    pub fn take_gram(&mut self, id: u32, want: usize, peek: bool) -> Result<Got, i64> {
        let s = self.get_mut(id).ok_or(EBADF)?;
        if let Some((from, gram)) = s.grams.front() {
            let got = Got {
                bytes: gram[..want.min(gram.len())].to_vec(),
                whole: gram.len(),
                from: *from,
            };
            if !peek {
                s.grams.pop_front();
            }
            return Ok(got);
        }
        if s.error != 0 {
            return Err(mem::take(&mut s.error));
        }
        if s.rd_shut {
            return Ok(Got { bytes: Vec::new(), whole: 0, from: Addr::default() });
        }
        Err(EAGAIN)
    }
}

pub struct Got {
    pub bytes: Vec<u8>,
    /// The datagram's length before it was cut to fit.
    pub whole: usize,
    pub from: Addr,
}
