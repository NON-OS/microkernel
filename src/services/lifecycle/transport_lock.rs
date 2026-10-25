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

//! The per-capsule transport locks, taken without busy-spinning, and taken
//! over from a holder killed inside its round trip (L77e): parked or never
//! woken, it never drops its guard, and every later caller of that capsule
//! waited for good. A contender finding the holder dead and on no CPU's
//! tables takes the lock over; the dead request's late reply carries an old
//! request id, which the next round trip throws away.

extern crate alloc;

use alloc::vec::Vec;
use spin::{Mutex, MutexGuard};

/// (lock address, the pid that last took it), taken around every acquisition,
/// so a held lock's row names its holder.
pub(super) static HOLDERS: Mutex<Vec<(usize, u32)>> = Mutex::new(Vec::new());

/// The holder is off-core for its whole exchange, so a contender yields too
/// rather than spin: on one CPU with interrupts masked a spin pins the core
/// and the holder never runs to let go.
pub fn lock_yielding(lock: &'static Mutex<()>) -> MutexGuard<'static, ()> {
    let key = lock as *const Mutex<()> as usize;
    let me = crate::process::current_pid().unwrap_or(0);
    let mut round = 0u32;
    loop {
        {
            let mut holders = HOLDERS.lock();
            if let Some(guard) = lock.try_lock() {
                match holders.iter_mut().find(|h| h.0 == key) {
                    Some(h) => h.1 = me,
                    None => holders.push((key, me)),
                }
                return guard;
            }
            if holders
                .iter()
                .any(|&(k, pid)| k == key && super::transport_holder::never_runs_again(pid))
            {
                // SAFETY: the holder never runs again, so its guard is never
                // dropped, and no one else can take the lock while HOLDERS is.
                unsafe { lock.force_unlock() };
                continue;
            }
        }
        super::reply_wait::rest(round);
        round = round.saturating_add(1);
    }
}
