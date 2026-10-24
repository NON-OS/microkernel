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

//! The order of the power-off and the status it answers. The flush comes
//! first and the shutdown runs whatever the flush did: a cache the flush
//! could not write is what the shutdown notification may still save. Pure,
//! so the host proofs hold both.

use crate::error::{NvmeError, NvmeResult};
use crate::protocol::{E_IO, E_TIMEDOUT};

/// What a power-off did: the flush's reply status, and the shutdown's end.
pub struct PoweredOff {
    pub flushed: Result<(), i32>,
    pub shut: NvmeResult<()>,
}

/// Run `flush`, then `shut` whatever `flush` returned.
pub fn power_off<F, S>(flush: F, shut: S) -> PoweredOff
where
    F: FnOnce() -> Result<(), i32>,
    S: FnOnce() -> NvmeResult<()>,
{
    let flushed = flush();
    PoweredOff { flushed, shut: shut() }
}

impl PoweredOff {
    /// The reply: the flush's failure first, then a shutdown that ran out of
    /// time as E_TIMEDOUT, any other as E_IO, else 0.
    pub fn status(&self) -> i32 {
        match (self.flushed, self.shut) {
            (Err(status), _) => status,
            (Ok(()), Err(NvmeError::ShutdownTimeout)) => E_TIMEDOUT,
            (Ok(()), Err(_)) => E_IO,
            (Ok(()), Ok(())) => 0,
        }
    }
}
