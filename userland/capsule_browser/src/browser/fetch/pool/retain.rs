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

//! Which finished fetches leave a connection that can be used again.

use alloc::vec::Vec;

use super::idle::Idle;
use crate::browser::fetch::keep::{MAX_KEEP_BYTES, MAX_KEEP_USES};
use crate::browser::fetch::types::Fetch;
use crate::browser::http::response::frame_len;
use crate::browser::url::Scheme;

/// The connection of finished `f`, whose response is `raw`, if it can carry
/// another request: the response was framed exactly, nothing followed it,
/// and the server did not ask to close. Plain HTTP and TLS alike.
pub fn retain(f: &mut Fetch, raw: &[u8], now: i64) -> Option<Idle> {
    if !f.keep || f.post.is_some() || f.error.is_some() || f.keep_uses >= MAX_KEEP_USES {
        return None;
    }
    let frame = frame_len(raw)?;
    if frame != raw.len() || wants_close(raw) || f.buf.len() > MAX_KEEP_BYTES {
        return None;
    }
    let tls = f.tls.take().filter(|tls| tls.server_app.is_some());
    let https = f.url.scheme == Scheme::Https;
    if https != tls.is_some() {
        return None;
    }
    let (buf, consumed) = match https {
        true => (core::mem::take(&mut f.buf), f.rx_consumed + frame),
        false => (Vec::new(), 0),
    };
    let (host, port, handle) = (f.url.host.clone(), f.url.port, f.handle);
    let (tx_seq, used) = (f.tx_seq + 1, f.keep_uses + 1);
    Some(Idle { host, port, https, handle, tls, buf, consumed, tx_seq, used, since_ms: now })
}

/// Whether the response head says the connection ends with it: it asks to
/// close, or it is HTTP/1.0 and does not ask to be kept.
pub fn wants_close(raw: &[u8]) -> bool {
    let Some(sep) = raw.windows(4).position(|w| w == b"\r\n\r\n") else { return true };
    let Ok(head) = core::str::from_utf8(&raw[..sep]) else { return true };
    let mut lines = head.lines();
    let old = lines.next().is_some_and(|status| status.starts_with("HTTP/1.0"));
    let said = lines.map(str::to_ascii_lowercase).find(|l| l.starts_with("connection:"));
    match said {
        Some(value) => !value.contains("keep-alive") || value.contains("close"),
        None => old,
    }
}
