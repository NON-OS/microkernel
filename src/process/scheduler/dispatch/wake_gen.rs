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

//! One wake counter per pid, so a sleeper can refuse to sleep through a wake.

use core::sync::atomic::{AtomicU64, Ordering};

/*
 * One counter per pid, bumped by every wake whether or not it transitions the
 * process. A wake aimed at a Running target must not strip a sleep deadline,
 * which is right for timeouts and fatal for events: the receiver checks its
 * queue, the message and its wake land in that gap as a no-op, and the
 * receiver then sleeps on a queue that has data. Sleeping through a wake it
 * has not observed is the lost-wakeup race; the counter is what lets a
 * sleeper refuse to.
 *
 * A fixed atomic table, not a map. wake_process runs from the timer sweep in
 * interrupt context, and a map entry insert can allocate; an allocation there
 * spins on a heap lock the interrupted code may hold, with interrupts off,
 * and the machine freezes whole. Two pids more than one generation apart
 * sharing a slot is harmless: a shared bump can only ever refuse a sleep one
 * loop iteration early, never permit sleeping through a wake.
 */
const WAKE_SLOTS: usize = 1024;
static WAKE_GENERATION: [AtomicU64; WAKE_SLOTS] = [const { AtomicU64::new(0) }; WAKE_SLOTS];

pub(super) fn wake_slot(pid: u32) -> &'static AtomicU64 {
    &WAKE_GENERATION[pid as usize % WAKE_SLOTS]
}

/// The wake counter as of now. Read before checking the condition the sleep
/// waits on, then passed to `sleep_until_unless_woken`.
pub fn wake_token(pid: u32) -> u64 {
    wake_slot(pid).load(Ordering::Acquire)
}
