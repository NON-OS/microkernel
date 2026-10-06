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

//! A blocking futex over the scheduler's sleep/wake, so std Mutex, RwLock,
//! Condvar, Once and thread parking sleep under contention instead of
//! spinning a yield loop. The wait queue is keyed on (thread group,
//! address): the threads of one capsule share a futex, other capsules stay
//! isolated. A waiter's sleep is capped at `SAFETY_MS`, so a wake that races
//! the enqueue costs a little latency, never a lost wakeup: the std side
//! rechecks the atomic after every return, so the cap is a safety net, not a
//! correctness dependency.

mod queue;
mod wait;
mod waiters;
mod wake;

pub use wait::sys_futex_wait;
pub use wake::sys_futex_wake;
