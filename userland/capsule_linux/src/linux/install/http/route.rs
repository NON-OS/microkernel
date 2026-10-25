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


//! A request for an install, over the Anyone network (`Route::for_installs`),
//! whatever network the browser uses: Nym's exits rotate and end a long
//! stream part way, Anyone's circuits carry a whole package. The mirror is
//! named, and the Anyone exit resolves the name, so an install names nothing
//! in the clear. On a cold boot Anyone takes minutes to build its first
//! circuit, so an install waits for it, up to three minutes, saying how far
//! it has got (anyone.rs, the fetcher's own rule). A network that does
//! not come up, or a stream it cannot open, is a refusal with its reason; no
//! failure is answered by trying another way. The reading is route_read.rs.

use alloc::vec::Vec;

use nonos_route_link::{Route, RouteStream};

use super::super::route_read::{read_reply, Reply};
use super::stream::Stream;

/// The reply to `request`, sent to `host`:`port` on the chosen route, read
/// until `done` says it is whole. None when the route is down, the stream
/// does not open or breaks before anything came, or more than `max` bytes
/// come.
pub(super) fn exchange(
    host: &str,
    port: u16,
    request: &[u8],
    max: usize,
    done: &dyn Fn(&[u8]) -> bool,
) -> Option<Vec<u8>> {
    /* Nothing reaches a network from a family that holds a model. */
    if crate::linux::file::models::held() {
        return refused(host, "this family holds a model");
    }
    let mut stream = match RouteStream::connect(Route::for_installs(), host, port) {
        Ok(s) => Stream(s),
        Err(why) => return refused(host, why),
    };
    if let Err(why) = stream.0.write_all(request) {
        return refused(host, why);
    }
    match read_reply(&mut stream, max, done) {
        Reply::Got(bytes) => Some(bytes),
        Reply::Nothing => refused(host, "nothing came back"),
        Reply::TooLarge => refused(host, "the reply is larger than any index or package"),
    }
}

/// Why `host` gave nothing, on the log, as raw.rs says it for a socket.
fn refused<T>(host: &str, why: &str) -> Option<T> {
    let line = alloc::format!("[LINUX] mirror {host}: {why}\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    None
}
