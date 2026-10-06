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

/*
 * No saved context yet: the task was woken on another CPU while the CPU it
 * runs on had not finished yielding it. Parking it Sleeping with no deadline
 * lost it for good, since the wake had already been spent. Park it on a one
 * tick deadline instead, so the sweep retries once its context is saved,
 * without the core re-picking it in a loop.
 */
pub(super) fn retry_unsaved(pid: u32) {
    let retry_ms = crate::time::timestamp_millis().saturating_add(1);
    crate::process::scheduler::dispatch::sleep_until(pid, retry_ms);
}
