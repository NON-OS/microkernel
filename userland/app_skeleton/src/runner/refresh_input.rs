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

use nonos_libc::mk_uptime_ms;

use crate::app::App;
use crate::discover::Peers;
use crate::setup::ensure_input_subscription;

use super::boot::BootedApp;

// Milliseconds between unconditional re-subscribe heartbeats: cheap enough to
// run forever and short enough that input recovers quickly. Measured in time,
// not frames, because a paced frame loop may sleep a long while between frames.
const RESUBSCRIBE_MS: i64 = 2000;

pub(super) fn refresh_input<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
) {
    // Re-assert the subscription on a heartbeat, not only after a failed
    // first attempt. The input router can drop a subscription out from under
    // us (dead-pid purge, router restart, table churn); without a periodic
    // re-assert the app would go input-deaf for the rest of the session.
    let now = mk_uptime_ms();
    if !booted.input_ready || beat_left(booted, now) == 0 {
        booted.input_ready =
            ensure_input_subscription(peers.input_router, &booted.manifest, request_id);
        booted.input_beat_ms = now;
    }
}

/// Milliseconds until the next heartbeat is due; zero when it is due now.
pub(super) fn beat_left<A: App>(booted: &BootedApp<A>, now: i64) -> i64 {
    let since = now.saturating_sub(booted.input_beat_ms);
    if since < 0 {
        return 0;
    }
    RESUBSCRIBE_MS.saturating_sub(since).max(0)
}
