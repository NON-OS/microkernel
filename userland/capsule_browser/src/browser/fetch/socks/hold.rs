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

//! Waiting for a network that is still coming up, or a proxy with no room.
//!
//! net.anon answers a CONNECT with "network unreachable" (3) until it has a
//! directory, a guard and a circuit, which on a cold boot takes minutes, and
//! net.socks5 does the same until net.nym is up. The browser failed the page
//! at the first such answer, so the first page after boot always failed,
//! and the reader had to guess when to try again. A proxy that turns the
//! greeting away because every place is taken is the same kind of "not
//! now". Now the fetch asks again, a second after each refusal, on a new
//! conversation, until the network takes it or HOLD_MS has passed since the
//! first refusal; the status line says how far the network has got
//! meanwhile (`net::mixnet::status`), and Stop ends the wait as it ends any
//! other.
//!
//! Pure; the proofs hold it.

use super::super::proxy_fault::{NOT_READY, PROXY_FULL};
use super::super::types::{Fetch, Hold, Phase, Wait};
use super::super::wire::Wire;

/// The longest a fetch waits for its network, from the first refusal.
pub const HOLD_MS: i64 = 180_000;

/// The pause before asking again.
pub const GAP_MS: i64 = 1_000;

/// The proxy said not now, for `why`: ask again after GAP_MS on a new
/// conversation, or stop once HOLD_MS has passed since the first refusal.
pub fn again<W: Wire>(w: &mut W, f: &mut Fetch, why: Wait) {
    let now = w.now_ms();
    let since = f.hold.map_or(now, |h| h.since);
    if now.wrapping_sub(since) >= HOLD_MS {
        return f.stop(match why {
            Wait::NotYet => NOT_READY,
            Wait::Full => PROXY_FULL,
        });
    }
    /* The proxy ended the conversation with its refusal; a new one begins
     * with its own reset, on whichever stream is free. */
    w.close(f.handle);
    let Ok(handle) = w.open(f.way) else {
        return f.stop(f.way.network().absent());
    };
    f.handle = handle;
    f.socks.clear();
    f.phase = Phase::SocksHello;
    f.hold = Some(Hold { why, since });
    f.not_before = now.saturating_add(GAP_MS);
}
