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

//! A caller giving up its CPU for one round of a wait: a yield, then a
//! tick's sleep once plain yields have had their chance.

/// Give the CPU away for one round, by sleeping a tick once plain yields
/// have had their chance. A caller with no process can only yield.
pub fn rest(round: u32) {
    rest_since(round, super::reply_wait::wake_mark());
}

pub(super) fn rest_since(round: u32, mark: u64) {
    // No listed spin lock may be held where a caller gives up its CPU (L81b).
    super::rest_mark::check();
    let pid = match crate::process::current_pid() {
        Some(pid) if round >= super::reply_wait::FAST_ROUNDS => pid,
        _ => {
            crate::sched::yield_now();
            return;
        }
    };
    let wake = crate::time::timestamp_millis().saturating_add(1);
    crate::sched::sleep_until_unless_woken(pid, wake, mark);
    crate::sched::yield_now();
}
