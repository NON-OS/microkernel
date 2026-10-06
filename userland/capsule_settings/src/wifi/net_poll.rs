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

//! How often the Wi-Fi and Network pages ask the DHCP client for its lease.
//!
//! The question is asked on the window's own thread. While a join is out the
//! page ticks every 100 ms, and the DHCP client is exactly then busy getting
//! an address, which it does before it answers anything else: each question
//! waited out its whole reply timeout, and the window stopped responding for
//! as long as the address took. The lease is asked at most once a second,
//! with a short timeout, and the page shows the last answer between.

/// The least time between two lease questions.
pub const NET_POLL_MS: i64 = 1_000;

/// How long one lease question waits for its answer. The DHCP client answers
/// a status question at once when it is free; one that is busy is asked
/// again a second later rather than waited on.
pub const LEASE_TIMEOUT_MS: u64 = 250;

/// Whether the lease may be asked at uptime `now_ms`, last asked at `last`
/// (None: not yet asked in this window).
pub fn net_poll_due(last: Option<i64>, now_ms: i64) -> bool {
    match last {
        None => true,
        Some(at) => now_ms < at || now_ms.saturating_sub(at) >= NET_POLL_MS,
    }
}
