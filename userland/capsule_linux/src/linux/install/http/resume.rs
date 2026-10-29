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

//! A pinned content tar fetched whole when a stream breaks partway. One
//! GET brings what it can; a reply that stops short is asked again on a
//! fresh stream of the same route from the byte held (`Range: bytes=N-`),
//! and a 206 from there is added to it (part.rs). A mirror that ignores the
//! Range and answers 200 starts it over. An ask after the first that
//! brings no byte ends it; the caller holds the whole to the pin.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::http_reply::complete;
use super::get::{request, said, MAX_BODY};
use super::part::part;
use super::route::exchange;

/// Asks after the first, each from where the last one stopped.
pub const RESUMES: usize = 8;

/// The whole of `path` at `at`, with `host` as the Host line, or None.
pub fn whole(at: &str, port: u16, host: &str, path: &str) -> Option<Vec<u8>> {
    let (mut got, mut total) = (Vec::new(), None);
    for ask in 0..=RESUMES {
        let had = got.len();
        let reply = exchange(at, port, ranged(path, host, had).as_bytes(), MAX_BODY, &complete);
        let Some(p) = reply.and_then(|raw| part(raw, had)) else {
            continue;
        };
        if p.total > MAX_BODY || total.is_some_and(|t| t != p.total) {
            break;
        }
        total = Some(p.total);
        match p.from {
            0 => got = p.bytes,
            _ => got.extend_from_slice(&p.bytes),
        }
        if got.len() == p.total || (ask > 0 && got.len() <= had) {
            break;
        }
        resumed(at, got.len(), p.total);
    }
    let done = total.filter(|t| got.len() == *t).map(|_| got);
    said(at, path, done.as_deref());
    done
}

/// The GET for `path` from byte `from`; from 0 it is the plain one.
fn ranged(path: &str, host: &str, from: usize) -> String {
    match from {
        0 => request(path, host),
        _ => format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: nonos\r\nRange: bytes={from}-\r\n\r\n"),
    }
}

fn resumed(at: &str, have: usize, total: usize) {
    let line = format!("[LINUX] mirror {at}: stopped at {have} of {total} bytes, asking on\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
