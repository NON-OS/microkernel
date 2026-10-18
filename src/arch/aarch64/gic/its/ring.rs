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

//! The command queue's arithmetic, apart from the memory and registers, so
//! the host proofs can drive it against a model ITS (IHI 0069, 5.2: the ITS
//! reads 32-byte commands from `GITS_CREADR` up to `GITS_CWRITER`, wrapping
//! at the end of the queue, and stops with `CREADR.Stalled` on an error).
//!
//! Every batch starts on an empty queue and ends in a SYNC waited on until
//! the ITS has read it, so a batch of at most one slot fewer than the queue
//! holds can never write over a command the ITS has not read. That holds
//! only while every batch has finished: once one has not, the ITS stopped
//! somewhere the kernel does not know, and the next batch could land on
//! commands still unread. So a failed batch closes the queue for good.

use super::ring_types::{Progress, Refused, BYTES, CREADR_STALLED, MAX_BATCH, QUEUE_OFFSET, SLOT};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Ring {
    write: u64,
    closed: bool,
}

impl Ring {
    /// Where the next batch's `count` commands go, the SYNC last, as the
    /// offset of the first; each further one is `SLOT` on, modulo `BYTES`.
    /// The ring moves past them: the value for `GITS_CWRITER` is `write()`.
    pub(crate) fn place(&mut self, count: usize) -> Result<u64, Refused> {
        if self.closed {
            return Err(Refused::Closed);
        }
        if count == 0 || count > MAX_BATCH {
            return Err(Refused::TooLong);
        }
        let start = self.write;
        self.write = (start + count as u64 * SLOT) % BYTES;
        Ok(start)
    }

    pub(crate) fn write(&self) -> u64 {
        self.write
    }

    /// What a `GITS_CREADR` value says about the batch just posted.
    pub(crate) fn progress(&self, creadr: u64) -> Progress {
        if creadr & CREADR_STALLED != 0 {
            Progress::Stalled
        } else if creadr & QUEUE_OFFSET == self.write {
            Progress::Done
        } else {
            Progress::Pending
        }
    }

    /// The batch did not finish: no further batch is placed.
    pub(crate) fn close(&mut self) {
        self.closed = true;
    }
}
