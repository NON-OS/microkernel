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

//! The command queue's shape and what a batch can come to (`ring`).

pub(crate) const BYTES: u64 = 0x1000;
pub(crate) const SLOT: u64 = 32;
/// The most commands one batch may hold, its SYNC included.
pub(crate) const MAX_BATCH: usize = (BYTES / SLOT) as usize - 1;
/// `GITS_CREADR` and `GITS_CWRITER` bits 19:5: a byte offset into the queue.
pub(crate) const QUEUE_OFFSET: u64 = 0xf_ffe0;
pub(crate) const CREADR_STALLED: u64 = 1 << 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Refused {
    /// More commands than one batch may hold.
    TooLong,
    /// An earlier batch did not finish.
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Progress {
    Done,
    Pending,
    Stalled,
}
