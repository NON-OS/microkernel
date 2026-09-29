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

//! A request on a connection kept from an earlier one.

use super::pool::Idle;
use super::types::{Fetch, Phase};
use super::wire::Wire;
use crate::browser::http;
use crate::browser::tls13;
use crate::browser::url::Url;

/*
 * The spent ciphertext and plaintext are dropped first, so the fetch reads
 * this response from the start of its buffer and the reader carries on at
 * the record sequence the connection has reached.
 */
/// A GET of `url` sent on a kept connection, in this call. `None` if the
/// connection would not take it; it is closed, and the caller opens anew.
pub fn reuse<W: Wire>(w: &mut W, mut idle: Idle, url: Url) -> Option<Fetch> {
    let req = http::request::build_keep_alive(&url);
    let bytes = match idle.tls.as_mut() {
        Some(tls) => {
            tls.reader.compact(&mut idle.buf, idle.consumed);
            let app = tls.server_app.as_ref();
            app.and_then(|app| tls13::application_request(app, idle.tx_seq, req.as_bytes()))
        }
        None => Some(req.into_bytes()),
    };
    if bytes.is_none_or(|b| w.send(idle.handle, &b).is_err()) {
        w.close(idle.handle);
        return None;
    }
    let mut f = Fetch::new(url, idle.handle, Phase::ReadBody, w.now_ms());
    f.buf = idle.buf;
    f.tls = idle.tls;
    f.tx_seq = idle.tx_seq;
    f.keep_uses = idle.used;
    f.keep = true;
    Some(f)
}
