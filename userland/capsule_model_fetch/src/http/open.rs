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
 * Opening a file at a mirror from a byte on: a connection through the
 * route, TLS with the chain checked for the host, the GET, and the answer
 * judged. A redirect to another HTTPS URL is followed; a plain HTTP one is
 * not, since it would send the request where anyone on the path can answer.
 */

use alloc::string::String;

use nonos_http::parse_url;
use nonos_tls::stream::connect;
use nonos_tls::{rtc_now, SessionError};

use super::body::Body;
use super::fault::Fault;
use super::head::parse;
use super::head_read::head;
use super::request::{next, request};
use crate::net::Route;

const REDIRECTS: usize = 5;

/* `url`'s file from byte `from`, which must be `bytes` long in all. */
pub fn open(route: Route, url: &str, from: u64, bytes: u64) -> Result<Body, Fault> {
    let mut at = String::from(url);
    for _ in 0..=REDIRECTS {
        let u = parse_url(&at).ok_or(Fault::Unusable("a mirror URL is not https://host/path"))?;
        let mut link = crate::net::connect(route, &u.host, 443)?;
        let mut tls = connect(&mut link, u.host.as_bytes(), rtc_now()).map_err(|e| match e {
            SessionError::Certificate => {
                Fault::Unusable("the mirror's certificate does not verify")
            }
            _ => Fault::Net("the TLS handshake with the mirror failed"),
        })?;
        let ask = request(&u.host, &u.path, from);
        tls.write_all(&mut link, ask.as_bytes())
            .map_err(|_| Fault::Net("the request was not sent"))?;
        let (raw, first) = head(&mut tls, &mut link)?;
        let h = parse(&raw).ok_or(Fault::Unusable("the mirror's answer is not HTTP"))?;
        let skip = match (h.status, h.range, h.length) {
            (301 | 302 | 303 | 307 | 308, _, _) => {
                at = next(&u.host, h.location.as_deref())?;
                continue;
            }
            _ if h.coded => return Err(Fault::Unusable("the mirror sent the file transfer-coded")),
            (206, Some((first_byte, total)), _) if first_byte == from && total == bytes => 0,
            (200, _, Some(len)) if len == bytes => from,
            (206 | 200, _, _) => {
                return Err(Fault::Unusable("the mirror's file is not the pinned length"))
            }
            (code, _, _) => return Err(Fault::Status(code)),
        };
        return Ok(Body { tls, link, first, skip });
    }
    Err(Fault::Unusable("the mirror redirected too many times"))
}
