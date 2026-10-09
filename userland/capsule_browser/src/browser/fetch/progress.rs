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

use alloc::string::String;

use crate::browser::fetch::types::{Fetch, Phase, Wait};
use crate::browser::net::mixnet::{still, Heard, Network};

/// A short human label for the current fetch phase, shown on the loading
/// screen so the reader sees real progress instead of a static "loading".
pub fn phase_label(phase: Phase) -> &'static str {
    match phase {
        Phase::Connecting | Phase::SocksHello | Phase::SocksMethod | Phase::SocksConnect => {
            "Connecting to"
        }
        Phase::TlsHello | Phase::TlsFlight => "Securing",
        Phase::SendReq => "Requesting",
        Phase::ReadBody => "Downloading",
        Phase::Decrypt | Phase::Done => "Rendering",
        Phase::Error => "Error",
    }
}

/// The status line for a navigation in flight: the phase and the host. It
/// changes only when the phase does, so a tick that moved nothing has
/// nothing new to draw.
pub fn status(f: &Fetch) -> String {
    alloc::format!("{} {}", phase_label(f.phase), f.url.host)
}

/// How long a connection through a proxy may sit in its handshake before
/// the status line says what the network is doing.
pub const QUIET_MS: i64 = 2_000;

/// Whether a navigation at `now` is waiting on its network rather than on
/// the site: in the SOCKS handshake, held for a network still connecting or
/// a proxy with no room, or quiet there for QUIET_MS. The proxy is asked
/// how far its network has got only then.
pub fn waiting(f: &Fetch, now: i64) -> bool {
    let socks = matches!(f.phase, Phase::SocksHello | Phase::SocksMethod | Phase::SocksConnect);
    socks && f.way.proxied() && (f.hold.is_some() || now.wrapping_sub(f.started_ms) >= QUIET_MS)
}

/// The status line for a navigation at `now`, with what its proxy said of
/// its network when it was asked. Waiting on the network, the line says
/// what the network is doing and for how long the page has waited, so a
/// cold network reads as working, not hung: "Connecting to example.org:
/// Anyone is still building its circuit (step 5 of 5), 40 s".
pub fn status_at(f: &Fetch, now: i64, heard: Option<Heard>) -> String {
    if !waiting(f, now) {
        return status(f);
    }
    let net = f.way.network();
    let secs = now.wrapping_sub(f.hold.map_or(f.started_ms, |h| h.since)).max(0) / 1_000;
    let doing = match (still(net, heard), f.hold.map(|h| h.why)) {
        (_, Some(Wait::Full)) => alloc::format!(
            "{} is serving as many connections as it can; waiting for room",
            proxy_name(net)
        ),
        (Some(line), _) => line,
        (None, Some(Wait::NotYet)) => alloc::format!("waiting for the {} to connect", net.label()),
        (None, None) => alloc::format!("through the {}", net.label()),
    };
    alloc::format!("{} {}: {}, {} s", phase_label(f.phase), f.url.host, doing, secs)
}

/// The service that carries `net`.
fn proxy_name(net: Network) -> &'static str {
    match net {
        Network::Anyone => "net.anon",
        Network::Nym | Network::Direct => "net.socks5",
    }
}
