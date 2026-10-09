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

/*
 * The GET a mirror is sent, and where a redirect sends it next. Host and
 * path come from `nonos_http::parse_url`, which takes printable ASCII with
 * no space, so neither can end a header.
 */

use alloc::format;
use alloc::string::String;

use super::fault::Fault;

pub fn request(host: &str, path: &str, from: u64) -> String {
    format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: nonos-model-fetch\r\n\
         Accept: */*\r\nAccept-Encoding: identity\r\nRange: bytes={from}-\r\n\
         Connection: close\r\n\r\n"
    )
}

/* Where a redirect points: an HTTPS URL, or a path on the same host. */
pub fn next(host: &str, location: Option<&str>) -> Result<String, Fault> {
    match location {
        Some(l) if l.starts_with("https://") => Ok(String::from(l)),
        Some(l) if l.starts_with('/') => Ok(format!("https://{host}{l}")),
        _ => Err(Fault::Unusable("the mirror redirected somewhere other than HTTPS")),
    }
}
