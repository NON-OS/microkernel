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

use alloc::format;
use alloc::vec::Vec;
use nonos_tls::{exchange, rtc_now};

use super::http::parse;
use super::tls_io::TcpIo;
use crate::tcp_client;

const HTTPS_PORT: u16 = 443;

/// One attempt at `ip`, authenticated as `host`.
pub(super) fn fetch_at(
    tcp_port: u32,
    ip: [u8; 4],
    host: &str,
    path: &str,
    max: usize,
) -> Result<Vec<u8>, u16> {
    crate::trace::say(b"fetch: connecting");
    let stream = tcp_client::connect(tcp_port, ip, HTTPS_PORT)?;
    tcp_client::wait_established(tcp_port, stream)?;
    crate::trace::say(b"fetch: handshaking");

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: nonos-nym\r\nAccept: application/json\r\nConnection: close\r\n\r\n",
        path, host
    );
    let mut io = TcpIo::new(tcp_port, stream);
    /*
     * The clock the chain is judged against, as YYYYMMDDhhmmss. Zero means the
     * wall clock was never set and every certificate reads as expired, which
     * is indistinguishable at the error code from a genuinely bad chain;
     * logging the value tells the two apart without a second boot.
     */
    let now = rtc_now();
    crate::trace::say_num(b"fetch: now", now);
    let raw = exchange(&mut io, host, request.as_bytes(), now, max);
    let stage = super::https_stage::stage(&raw);
    crate::trace::say_two(b"fetch: ok-len-or-err", stage, io.overran() as u64);
    let _ = tcp_client::close(tcp_port, stream);
    let body = raw.map_err(|_| if io.overran() { 22u16 } else { 20u16 })?;
    /*
     * A short answer is a redirect or an error page, not a node list, and
     * parsing it would find no objects and report an empty directory rather
     * than a wrong address.
     */
    if !super::https_stage::status_ok(&body) {
        return Err(20);
    }
    parse::body(&body)
}
