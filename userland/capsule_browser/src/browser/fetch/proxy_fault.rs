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

//! A conversation with a proxy that ended for a reason of the proxy's own.
//!
//! Every way a proxy could stop answering read as a closed stream, and a
//! closed stream as "the exit closed the connection": a proxy restarted
//! under the page, a proxy no longer at its port, and an answer that could
//! not be read all blamed the site. The conversation now keeps why it broke
//! (`net::mixnet::Broke`), and the fetch is stopped with that instead.
//!
//! Pure; the proofs hold it.

use super::closed::EXIT_CLOSED;
use super::types::{Fetch, Phase};
use super::wire::Wire;
use crate::browser::net::mixnet::Broke;

/// The proxy is no longer at its port: it was stopped, or restarted and
/// took another. Tried again, the service is looked up afresh.
pub const PROXY_GONE: &str = "proxy gone";

/// The proxy answered with bytes that are not an answer.
pub const PROXY_GARBLED: &str = "proxy garbled";

/// The proxy holds no such conversation: it was restarted under it.
pub const PROXY_LOST: &str = "proxy lost";

/// The proxy ended the conversation before answering the SOCKS greeting or
/// request: both proxies answer those themselves, so the exit never had it.
pub const PROXY_ENDED: &str = "proxy ended";

/// The network was still connecting when the wait for it ran out
/// (`socks::hold`): nothing was sent.
pub const NOT_READY: &str = "network not ready";

/// The proxy had no room for another conversation all that while.
pub const PROXY_FULL: &str = "proxy full";

/// The code a fetch stops with when its proxy broke for `why`.
pub fn code(why: Broke) -> &'static str {
    match why {
        Broke::Gone => PROXY_GONE,
        Broke::Garbled => PROXY_GARBLED,
        Broke::Lost => PROXY_LOST,
    }
}

/// A fetch that stopped on a close, or on a send the proxy would not take,
/// is stopped with the proxy's own reason when it has one.
pub fn name<W: Wire>(w: &W, f: &mut Fetch) {
    let closed = matches!(
        f.error,
        Some(EXIT_CLOSED | PROXY_ENDED | "send failed" | "socks hello failed" | "socks connect failed")
    );
    if f.phase != Phase::Error || !closed {
        return;
    }
    if let Some(why) = w.broke(f.handle) {
        f.error = Some(why);
    }
}
