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

//! The status line (RFC 9112 4): HTTP/1.x, a space, three digits, then a
//! space and a reason or nothing.

use nonos_http::{parse_response, HttpError};

fn status(line: &str) -> Result<u16, HttpError> {
    parse_response(format!("{line}\r\nContent-Length: 0\r\n\r\n").as_bytes()).map(|r| r.status)
}

#[test]
fn ordinary_status_lines_are_read() {
    assert_eq!(status("HTTP/1.1 200 OK"), Ok(200));
    assert_eq!(status("HTTP/1.0 404 Not Found"), Ok(404));
    assert_eq!(status("HTTP/1.1 500"), Ok(500), "the reason may be absent");
    assert_eq!(status("HTTP/1.1 503 "), Ok(503), "or empty");
}

/// Anything in the separator places, a code outside 100 to 599, or a fourth
/// digit, is not a status line.
#[test]
fn malformed_status_lines_are_refused() {
    for line in [
        "HTTP/1.1X200 OK",
        "HTTP/1.1 200OK",
        "HTTP/1.1 2000 OK",
        "HTTP/1.1 099 Low",
        "HTTP/1.1 600 High",
        "HTTP/1.x 200 OK",
        "HTTP/1.1  200 OK",
    ] {
        assert_eq!(status(line), Err(HttpError::StatusLine), "{line:?}");
    }
}
