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

//! A guest's reads on a stream this capsule carries: the SOCKS endpoint's
//! replies first, then the stream's own bytes. A read moves the stream on
//! first (drive.rs), as a write does.

use alloc::vec::Vec;

use super::drive::{drive, handle};
use super::entry::Phase;
use super::table;
use crate::linux::abi::errno::{self, EAGAIN, EBADF};

/// Up to `want` bytes: the endpoint's replies first, then the stream's.
pub fn take(id: u32, want: usize, peek: bool) -> Result<Vec<u8>, u64> {
    let h = handle(id).ok_or(errno::fail(EBADF))?;
    if want == 0 {
        return Ok(Vec::new());
    }
    drive(id, false);
    let got = table::with(h, |e| {
        if !e.out.is_empty() {
            let n = want.min(e.out.len());
            let said = e.out[..n].to_vec();
            if !peek {
                e.out.drain(..n);
            }
            return Ok(said);
        }
        match &mut e.phase {
            Phase::Open(link) => link.take(want, peek),
            Phase::Closed => Ok(Vec::new()),
            Phase::Failed(errno) => Err(*errno),
            _ => Err(EAGAIN),
        }
    });
    got.unwrap_or(Err(EAGAIN)).map_err(errno::fail)
}
