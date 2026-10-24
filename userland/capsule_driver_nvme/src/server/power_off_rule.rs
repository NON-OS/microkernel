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

//! The status the power-off answers. No flush comes first: a normal
//! shutdown (CC.SHN 01b, NVMe 2.0 3.6.2) is the controller's word that its
//! volatile state, the write cache with it, is on the medium, and a flush
//! ahead of it would hold the notification behind the I/O queue's 30 s
//! window, past the kernel's budget for this driver. Pure, so the host
//! proofs hold it.

use crate::error::{NvmeError, NvmeResult};
use crate::protocol::{E_IO, E_TIMEDOUT};

/// The reply for a power-off whose shutdown ended in `shut`: one that ran
/// out of time as E_TIMEDOUT, any other failure as E_IO, else 0.
pub fn status(shut: NvmeResult<()>) -> i32 {
    match shut {
        Ok(()) => 0,
        Err(NvmeError::ShutdownTimeout) => E_TIMEDOUT,
        Err(_) => E_IO,
    }
}
