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

//! Making a connection without holding the window while it is made. The
//! connect used to be one call that waited up to nine seconds, and nothing
//! was drawn or answered meanwhile. Now it is started and polled once a
//! tick, and the first bytes go out in the tick it is accepted.

use super::types::Fetch;
use super::wire::{HostFail, Resolved, Wire, WRITABLE};
use crate::browser::net::parse_ipv4;

/// No DNS server is reachable: every name fails alike until the network is
/// back, so it is said as the network, never as the spelling, and not tried
/// again at once (net.sockets refuses lookups for a few seconds after one).
pub(in crate::browser::fetch) const NO_DNS: &str = "dns unreachable";

/// Move a connecting fetch on as far as it can go without waiting.
pub(in crate::browser::fetch) fn connect<W: Wire>(w: &mut W, f: &mut Fetch) {
    let Some(ip) = address(w, f) else { return };
    let Some(d) = f.dial.as_mut() else { return };
    if !d.sent {
        if w.connect_nb(f.handle, ip, d.port).is_err() {
            return f.stop("connect failed");
        }
        d.sent = true;
    }
    match w.poll(f.handle) {
        Ok(bits) if bits & WRITABLE != 0 => connected(f),
        Ok(_) => {}
        Err(_) => f.stop("connect failed"),
    }
}

/// The address to connect to, once known. A name no cache holds is resolved
/// only after one tick has let the window show what it waits for.
fn address<W: Wire>(w: &mut W, f: &mut Fetch) -> Option<[u8; 4]> {
    let d = f.dial.as_mut()?;
    if d.ip.is_none() {
        d.ip = parse_ipv4(&d.host).or_else(|| w.cached(&d.host));
    }
    if d.ip.is_some() || !d.shown {
        d.shown = true;
        return d.ip;
    }
    let found = w.resolve(&d.host);
    if let Resolved::Ip(ip) = found {
        d.ip = Some(ip);
        return d.ip;
    }
    /* net.dns cannot be asked, so net.sockets resolves, and waits. */
    let joined = match found {
        Resolved::Unavailable => Some(w.connect_host(f.handle, &d.host, d.port)),
        _ => None,
    };
    match (found, joined) {
        (_, Some(Ok(()))) => connected(f),
        (Resolved::NoDns, _) | (_, Some(Err(HostFail::NoDns))) => f.stop(NO_DNS),
        (Resolved::Unknown, _) => f.stop("dns failed"),
        _ => f.stop("connect failed"),
    }
    None
}

fn connected(f: &mut Fetch) {
    if let Some(d) = f.dial.take() {
        f.phase = d.then;
    }
}
