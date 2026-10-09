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

//! Frame pacing for the `run` frame loop. Between frames the loop blocks on
//! the service inbox until a message lands or the next deadline comes due, so
//! an idle window costs no CPU while input still wakes it at once. A message
//! that ends the wait stays in the receive buffer and is held for the next
//! drain, so nothing is lost or reordered.

use nonos_libc::{mk_display_vsync_wait, mk_ipc_recv_from, mk_yield};

use super::held::Held;

const SERVICE_INBOX: u64 = 0;
const ETIMEDOUT: i64 = -110;
/// Longest single wait, so a stale deadline can never park the loop for long.
const MAX_WAIT_MS: i64 = 1000;
/// Retry cadence while the window is unprimed or input is unsubscribed: about
/// one vblank, the pace these retries have under `run_loop`.
const RETRY_MS: i64 = 16;
/// Cap while the app reports busy: the millisecond an empty drain has always
/// slept, so pending work keeps the cadence it had before pacing.
const BUSY_MS: i64 = 1;

/// Milliseconds to block: the least of the time left to the next tick and to
/// the input heartbeat, capped as above. Zero means work is already due.
pub(super) fn wait_ms(tick_left: i64, beat_left: i64, retrying: bool, busy: bool) -> i64 {
    let mut ms = tick_left.min(beat_left).min(MAX_WAIT_MS);
    if retrying {
        ms = ms.min(RETRY_MS);
    }
    if busy {
        ms = ms.min(BUSY_MS);
    }
    ms.max(0)
}

/// Blocks for at most `ms` or until a message lands. `holding` says a message
/// from an earlier wait is still unread in `rx`; it must not be overwritten.
pub(super) fn block(rx: &mut [u8], ms: i64, holding: bool) -> Option<Held> {
    if ms <= 0 {
        let _ = mk_yield();
        return None;
    }
    if holding {
        let _ = mk_display_vsync_wait(0);
        return None;
    }
    let mut sender = 0u32;
    let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), ms as u64, &mut sender);
    if n > 0 {
        return Some(Held { len: n as usize, sender });
    }
    if n != ETIMEDOUT {
        /*
         * The inbox answered with an error at once; a vblank keeps a broken
         * inbox from turning the wait back into a spin.
         */
        let _ = mk_display_vsync_wait(0);
    }
    None
}
