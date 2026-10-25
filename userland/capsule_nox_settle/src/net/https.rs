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

//! One HTTPS exchange over Route::for_rpc(): the wallet's, the Nym mixnet or
//! the Anyone network, never Direct but for the local test chain's one name in
//! a `devnet` build, and a route that is down is ENETDOWN. nonos_tls
//! checks the host's chain before the request is sealed. Only a 200 is an
//! answer: a redirect is refused, never followed to another host.

use alloc::vec::Vec;

use nonos_eth_rpc::{http_answer, http_post, Http};
use nonos_route_link::{Route, RouteStream};
use nonos_settle_proto::{EFBIG, EIO, ENETDOWN, EPROTO};
use nonos_tls::{exchange, rtc_now, SessionError};

use super::io::RouteIo;

/// An RPC exchange may take this long in all.
const RPC_TOTAL_MS: u64 = 90_000;
/// A release fetch may take this long in all.
pub(super) const FETCH_TOTAL_MS: u64 = 300_000;
/// What an HTTP head may add to a body.
const HEAD_MAX: usize = 16 * 1024;

pub(super) fn https(host: &str, req: &[u8], limit: usize, ms: u64) -> Result<Vec<u8>, i32> {
    let route = Route::for_rpc(host);
    if matches!(route, Route::Down(_)) {
        return Err(ENETDOWN);
    }
    let stream = RouteStream::connect_rpc(route, host, 443).map_err(|_| EIO)?;
    let mut io = RouteIo::new(stream, route.patience_ms(), ms);
    let max = limit.saturating_add(HEAD_MAX);
    let plain = exchange(&mut io, host, req, rtc_now(), max).map_err(|e| match e {
        SessionError::TooLarge => EFBIG,
        SessionError::Certificate => EPROTO,
        _ => EIO,
    })?;
    match http_answer(&plain) {
        Http::Body(body) if body.len() <= limit => Ok(body),
        Http::Body(_) => Err(EFBIG),
        Http::Status(_) | Http::Incomplete => Err(EIO),
    }
}

/// A JSON-RPC POST to `host`; the answer's body, at most `limit` bytes.
pub fn post(host: &str, body: &[u8], limit: usize) -> Result<Vec<u8>, i32> {
    https(host, &http_post(host.as_bytes(), body), limit, RPC_TOTAL_MS)
}

/// A GET of `path` on `host`; the body, at most `limit` bytes.
pub fn get(host: &str, path: &str, limit: usize) -> Result<Vec<u8>, i32> {
    let mut req = Vec::with_capacity(host.len().saturating_add(path.len()).saturating_add(96));
    for part in ["GET ", path, " HTTP/1.1\r\nHost: ", host] {
        req.extend_from_slice(part.as_bytes());
    }
    req.extend_from_slice(b"\r\nAccept: */*\r\nConnection: close\r\n\r\n");
    https(host, &req, limit, FETCH_TOTAL_MS)
}
