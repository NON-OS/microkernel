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

//! Starting a fetch: on a fresh connection, or on a kept one.

use super::types::{Dial, Fetch, Phase};
use super::wire::Wire;
use crate::browser::net::mixnet::Way;
use crate::browser::url::{Scheme, Url};

/*
 * Every caller used to repeat open, blocking connect, pick a phase, build a
 * Fetch. Here the socket is opened and the fetch waits in Connecting; its
 * first step starts the connect, and the step that sees it accepted sends
 * the first flight: the SOCKS greeting, the ClientHello, or the request.
 * A proxy has nothing to connect, so its greeting goes on the first step.
 *
 * The way out is the host's own (`net::mixnet::way`), taken here once and
 * kept by the fetch: an .anyone host goes through net.anon whatever the
 * reader chose, and one that cannot go the way it must is refused with the
 * reason, never sent another way. A SOCKS proxy the reader set applies to
 * direct connections only; a network's own proxy is never stacked on it.
 * A conversation with a network's proxy is a stream of its own, and is
 * kept for the next request like a direct connection.
 */
/// Every stream at the proxy is in use by this browser.
pub const NO_STREAM: &str = "no free stream";

/// A fetch of `url`, through `proxy` when one is set.
pub fn open<W: Wire>(
    w: &mut W,
    url: Url,
    proxy: Option<(&str, u16)>,
) -> Result<Fetch, &'static str> {
    let way = w.way(&url.host);
    let handle = match way {
        Way::Refused(why) => return Err(why),
        /* Every stream the browser may hold at the proxy is held: the
         * proxy runs, and saying it does not would send the reader to look
         * for a fault that is not there. The pool waits its turn instead
         * (`pool::start`), and a navigation makes room first (`nav::load`). */
        Way::Proxy { .. } if !w.room(way) => return Err(NO_STREAM),
        Way::Proxy { net, .. } => w.open(way).map_err(|_| net.absent())?,
        Way::Direct => w.open(way).map_err(|_| "socket failed")?,
    };
    let now = w.now_ms();
    let socks = proxy.is_some() || way.proxied();
    let then = match (socks, url.scheme) {
        (true, _) => Phase::SocksHello,
        (false, Scheme::Https) => Phase::TlsHello,
        (false, Scheme::Http) => Phase::SendReq,
    };
    if way.proxied() {
        let mut f = Fetch::new(url, handle, then, now);
        f.way = way;
        /* Kept for the next request to the host, with its SOCKS handshake,
         * its tunnel and its TLS session: through a mixnet each of those is
         * seconds of round trips. */
        f.keep = true;
        return Ok(f);
    }
    let (host, port) = proxy.unwrap_or((url.host.as_str(), url.port));
    let dial = Dial::new(host, port, then);
    let mut f = Fetch::new(url, handle, Phase::Connecting, now);
    f.dial = Some(dial);
    f.keep = !socks;
    f.way = way;
    Ok(f)
}
