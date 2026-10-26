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

//! The fatal path's writer. A CPU that traps while it holds the port's lock,
//! or while another CPU that will never run again holds it, would spin in
//! with_serial_lock forever and the trap would never be named. So the lock
//! is taken if it frees within a bounded number of tries, and the line is
//! written either way: a line interleaved with another CPU's output can
//! still be read, a line never written cannot.

use core::sync::atomic::{AtomicBool, Ordering};

/// About 50 ms of spinning with `pause` on the machines NONOS runs on.
pub const TRIES: u32 = 1 << 20;

/// Write each of `parts` and a newline, holding the lock if `try_lock` gets
/// it within `tries`, and writing either way; never waits longer than that.
/// Once a line has waited the whole bound out, `lost` is set and later lines
/// try once only, so a dying machine is not slowed by every line it says;
/// the first line that gets the lock clears it.
pub fn write_bounded<G>(
    tries: u32,
    lost: &AtomicBool,
    mut try_lock: impl FnMut() -> Option<G>,
    mut write: impl FnMut(u8),
    parts: &[&[u8]],
) {
    let tries = if lost.load(Ordering::Relaxed) { tries.min(1) } else { tries };
    let mut guard = None;
    for _ in 0..tries {
        guard = try_lock();
        if guard.is_some() {
            break;
        }
        ::core::hint::spin_loop();
    }
    lost.store(tries > 0 && guard.is_none(), Ordering::Relaxed);
    parts.iter().flat_map(|p| p.iter()).chain(b"\r\n").for_each(|&ch| write(ch));
    drop(guard);
}
