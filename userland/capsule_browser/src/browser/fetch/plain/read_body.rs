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

use crate::browser::fetch::budget::Budget;
use crate::browser::fetch::constants::MAX_BODY;
use crate::browser::fetch::deadline::read_ms;
use crate::browser::fetch::tls;
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::fetch::wire::Wire;
use crate::browser::http::response::{has_headers, is_complete};
use crate::browser::net::drain::drain;

/*
 * A response with a Content-Length or chunked framing ends in the call that
 * brings its last byte, over TLS as in the clear: waiting for the server to
 * close, or for a quiet timer, held every response on a kept connection and
 * every one from a server that does not close at once. Only a response with
 * no framing at all ends on quiet, since a read cannot tell a closed socket
 * from an empty one.
 */
/// Read what has arrived of the response and end it if it is whole.
pub(in crate::browser::fetch) fn read_body<W: Wire>(
    w: &mut W,
    f: &mut Fetch,
    b: &Budget,
    until: i64,
) {
    let read = drain(w, f.handle, &mut f.buf, MAX_BODY, read_ms(w.now_ms(), until));
    if read.got > 0 {
        f.progress_ms = w.now_ms();
        f.received += read.got;
    }
    let secure = f.tls.is_some();
    let (headed, whole) = if secure {
        let Some(plain) = tls::plain(f) else { return f.stop("tls record failed") };
        (has_headers(plain), is_complete(plain))
    } else {
        (has_headers(&f.buf), is_complete(&f.buf))
    };
    if secure && tls::broken(f) {
        return f.stop("tls record failed");
    }
    let end = if secure { Phase::Decrypt } else { Phase::Done };
    if whole {
        f.phase = end;
    } else if read.full {
        f.stop("response too large");
    } else if headed && read.got == 0 && w.now_ms().wrapping_sub(f.progress_ms) > b.idle_ms {
        f.phase = end;
    }
}
