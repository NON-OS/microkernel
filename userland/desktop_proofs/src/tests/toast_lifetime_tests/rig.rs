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

//! The shell's idle loop, as the toasts see it.

use crate::shell_tick::{tick_due, wait_ms, wake_at, TICK_MS};
use crate::toast_state::toast::UptimeMs;
use crate::toast_state::toasts::ToastQueue;

/// The serve loop's longest wait on an idle inbox (runner/constants.rs).
pub(super) const RECV_BLOCK: u64 = 1000;

/// What the screen shows: the queue as of the last panel sync.
pub(super) struct Screen {
    pub(super) synced: u32,
    pub(super) shown: usize,
}

/// The serve loop with an inbox that stays empty: no pointer, no app, no
/// other repaint. Runs from uptime `now` until `until`, its tick running on
/// whole seconds of uptime as the shell's does from boot, and gives back the
/// uptime at which the panel was last seen to empty, if it did.
pub(super) fn idle_loop(
    q: &mut ToastQueue,
    s: &mut Screen,
    mut now: i64,
    until: i64,
) -> Option<i64> {
    let mut last_tick = now - now.rem_euclid(TICK_MS);
    let mut emptied = None;
    while now < until {
        let wake = wake_at(last_tick, q.next_expiry().map(|t| t.0));
        now += wait_ms(now, wake, RECV_BLOCK) as i64;
        if tick_due(now, last_tick) {
            last_tick = now;
        }
        q.expire(UptimeMs(now));
        if s.synced != q.generation() {
            s.synced = q.generation();
            let was = s.shown;
            s.shown = q.iter_live().count();
            if was != 0 && s.shown == 0 {
                emptied = Some(now);
            }
        }
    }
    emptied
}
