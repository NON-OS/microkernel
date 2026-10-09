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

//! One serial line per page load, so `log BROWSER` names how a page that did
//! not load ended: "[BROWSER] github.com: tls stopped (tls handshake refused)
//! after 12 s, 4 KB in, over Anyone". The page shows the same reason in
//! words; the line is for a machine where nobody wrote the words down.

use alloc::format;
use alloc::string::String;

use super::types::{Fetch, Phase};
use crate::browser::net::mixnet::Network;

/// The stage a fetch was in, as the line names it.
pub fn stage(p: Phase) -> &'static str {
    match p {
        Phase::Connecting => "connect",
        Phase::SocksHello | Phase::SocksMethod | Phase::SocksConnect => "proxy",
        Phase::TlsHello | Phase::TlsFlight => "tls",
        Phase::SendReq => "request",
        Phase::ReadBody => "response",
        Phase::Decrypt | Phase::Done => "page",
        Phase::Error => "stopped",
    }
}

/// The network, short.
pub fn over(net: Network) -> &'static str {
    match net {
        Network::Direct => "Direct",
        Network::Nym => "Nym",
        Network::Anyone => "Anyone",
    }
}

/// The status line of a response, "HTTP/1.1 200 OK", or what stands in.
pub fn status_line(raw: &[u8]) -> &str {
    let end = raw.iter().position(|&b| b == b'\r' || b == b'\n').unwrap_or(raw.len()).min(80);
    core::str::from_utf8(&raw[..end]).unwrap_or("an unreadable status line")
}

/// Bytes as the line says them.
pub fn size(n: usize) -> String {
    if n < 1024 {
        format!("{n} B")
    } else if n < 1024 * 1024 {
        format!("{} KB", n / 1024)
    } else {
        format!("{}.{} MB", n / (1024 * 1024), n % (1024 * 1024) * 10 / (1024 * 1024))
    }
}

/// The line for a load that ended at `now_ms` (the uptime clock its
/// deadlines run on), with `what` it came to.
pub fn line(f: &Fetch, now_ms: i64, what: &str) -> String {
    let secs = now_ms.wrapping_sub(f.started_ms).max(0) / 1000;
    format!(
        "[BROWSER] {}: {what} after {secs} s, {} in, over {}\n",
        f.url.host,
        size(f.received),
        over(f.net())
    )
}

/// What a load that stopped came to: the stage, the code, and the server's
/// alert when it sent one.
pub fn stopped(f: &Fetch, said: &str) -> String {
    let at = f.stopped_in.map_or("stopped", stage);
    let code = f.error.unwrap_or("no reason recorded");
    let alert = f.tls_alert.map(|a| format!(", alert {a}")).unwrap_or_default();
    if said.is_empty() || said == code {
        format!("{at} failed ({code}{alert})")
    } else {
        format!("{at} failed ({code}{alert}): {said}")
    }
}
