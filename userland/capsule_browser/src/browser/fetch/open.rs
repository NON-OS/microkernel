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
use crate::browser::url::{Scheme, Url};

/*
 * Every caller used to repeat open, blocking connect, pick a phase, build a
 * Fetch. Here the socket is opened and the fetch waits in Connecting; its
 * first step starts the connect, and the step that sees it accepted sends
 * the first flight: the SOCKS greeting, the ClientHello, or the request.
 * The mixnet has nothing to connect, so its greeting goes on the first step.
 */
/// A fetch of `url`, through `proxy` when one is set.
pub fn open<W: Wire>(
    w: &mut W,
    url: Url,
    proxy: Option<(&str, u16)>,
) -> Result<Fetch, &'static str> {
    let handle = w.open().map_err(|_| "socket failed")?;
    let now = w.now_ms();
    let socks = proxy.is_some() || w.mixnet();
    let then = match (socks, url.scheme) {
        (true, _) => Phase::SocksHello,
        (false, Scheme::Https) => Phase::TlsHello,
        (false, Scheme::Http) => Phase::SendReq,
    };
    if w.mixnet() {
        return Ok(Fetch::new(url, handle, then, now));
    }
    let (host, port) = proxy.unwrap_or((url.host.as_str(), url.port));
    let dial = Dial::new(host, port, then);
    let mut f = Fetch::new(url, handle, Phase::Connecting, now);
    f.dial = Some(dial);
    f.keep = !socks;
    Ok(f)
}
