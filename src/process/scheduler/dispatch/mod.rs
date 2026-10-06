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
 * PID run queue, sleep table, and wakeup hook for the scheduler. The
 * scheduling-event atomics this code increments live next door in
 * `super::preemption`.
 */

mod run_queue;
mod sleep;
mod sleep_enter;
mod sleep_table;
mod sweep;
pub mod try_wake;
mod wake;
mod wake_gen;
mod wakeup;

pub use run_queue::{
    add_to_run_queue, add_to_run_queue_front, is_in_run_queue, remove_from_run_queue,
};
pub use run_queue::{get_runnable_pids, runnable_process_count};
pub use sleep::{sleep_until, sleep_until_unless_woken};
pub use sleep_table::{get_remaining_sleep, is_sleeping};
pub use sweep::check_sleeping_processes;
pub use wake::wake_process;
pub use wake_gen::wake_token;
pub use wakeup::wakeup;
