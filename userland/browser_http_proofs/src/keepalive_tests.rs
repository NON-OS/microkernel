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

//! Whether a finished response leaves its connection open for the next.

use crate::browser::http::response::wants_close;
use crate::vectors::response;

#[test]
fn http10_and_connection_close_do_not_keep_the_connection() {
    let keep = response(&["HTTP/1.1 200 OK", "Content-Length: 0"], b"");
    let close =
        response(&["HTTP/1.1 200 OK", "Connection: keep-alive, Close", "Content-Length: 0"], b"");
    let old = response(&["HTTP/1.0 200 OK", "Content-Length: 0"], b"");
    let old_keep =
        response(&["HTTP/1.0 200 OK", "Connection: keep-alive", "Content-Length: 0"], b"");
    assert_eq!(
        [&keep, &close, &old, &old_keep].map(|r| wants_close(r)),
        [false, true, true, false]
    );
}
