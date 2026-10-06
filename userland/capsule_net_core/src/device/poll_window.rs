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

//! The window a poll may spend on the device, and what it may send in it.

use core::sync::atomic::{AtomicBool, AtomicI64, AtomicU32, Ordering};

use nonos_libc::mk_time_millis;

use super::budget::{POLL_WINDOW_MS, TX_FLOOR};

static POLL_DEADLINE: AtomicI64 = AtomicI64::new(0);
static TX_THIS_POLL: AtomicU32 = AtomicU32::new(0);
static IN_POLL: AtomicBool = AtomicBool::new(false);

/// Open a polling window. Driver traffic is only allowed inside one.
pub fn open_poll() {
    POLL_DEADLINE.store(mk_time_millis() + POLL_WINDOW_MS, Ordering::Relaxed);
    TX_THIS_POLL.store(0, Ordering::Relaxed);
    IN_POLL.store(true, Ordering::Relaxed);
}

/// Close the polling window, so nothing reaches the driver off the poll path.
pub fn close_poll() {
    POLL_DEADLINE.store(0, Ordering::Relaxed);
    IN_POLL.store(false, Ordering::Relaxed);
}

/// Whether there is still budget to spend on the device this poll.
pub fn poll_open() -> bool {
    mk_time_millis() < POLL_DEADLINE.load(Ordering::Relaxed)
}

/// Whether a frame the stack produced may go to the card now: inside a poll,
/// while the window is open or the poll has sent fewer than `TX_FLOOR`.
pub fn may_send() -> bool {
    if !IN_POLL.load(Ordering::Relaxed) {
        return false;
    }
    let sent = TX_THIS_POLL.fetch_add(1, Ordering::Relaxed);
    sent < TX_FLOOR || poll_open()
}
