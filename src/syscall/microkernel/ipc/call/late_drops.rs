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

//! The late replies the correlation check has discarded since boot, said
//! beside a silence (silence_table.rs): how many answers came too late. Each
//! is an answer all the same, so it ends the silence of the call it answers.

use core::sync::atomic::{AtomicU64, Ordering};

use super::verdict::stale_reply;

static DROPPED: AtomicU64 = AtomicU64::new(0);

/// Counts a message `caller`'s reply receive discarded, if it was a late
/// reply: the server answered that call after all.
pub(crate) fn note_drop(caller: u32, correlation: u64, want: u64) {
    if stale_reply(correlation, want) {
        DROPPED.fetch_add(1, Ordering::Relaxed);
        super::silence_table::replied(caller, correlation);
    }
}

pub(super) fn dropped() -> u64 {
    DROPPED.load(Ordering::Relaxed)
}
