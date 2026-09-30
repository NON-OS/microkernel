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

//! What `MkIrqWait` returns once the seq is read back.

/// Returned when the wait slept out its whole timeout with the seq unmoved.
/// Otherwise the call returns a negative errno on failure, or 0 (the seq
/// moved, or another wake cut the sleep short). Positive on purpose: a
/// caller that only tests for a negative errno keeps treating it as an
/// ordinary return, and one that spends a time budget can tell a slept-out
/// slice from an early wake.
const IRQ_WAIT_TIMED_OUT: i64 = 1;

/// `IRQ_WAIT_TIMED_OUT` only when the caller slept to `deadline` (set when
/// it slept at all) and `current` is still `last_seq`: an early wake (IPC, a
/// stray wake) and a seq that moved during or before the sleep both give 0.
pub(super) fn verdict(deadline: Option<u64>, current: u64, last_seq: u64) -> i64 {
    match deadline {
        Some(at) if current == last_seq && crate::time::timestamp_millis() >= at => {
            IRQ_WAIT_TIMED_OUT
        }
        _ => 0,
    }
}
