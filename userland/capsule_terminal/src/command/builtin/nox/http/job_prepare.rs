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

//! An `http` job set up: the url read, the route said, the request built.

use alloc::vec::Vec;

use nonos_http::RequestBuilder;
use nonos_route_link::Route;
use nonos_tls::rtc_now;

use super::super::url::parse_url;
use super::HttpJob;
use crate::mixnet::{Exchange, Stage};
use crate::term::state::State;

const USAGE: &[u8] =
    b"usage: http <url>   e.g. http example.com  |  http https://host/path  |  http host:8080";
const MAX_BODY: usize = 64 * 1024;
const AGENT: &str = "nonos-terminal";
/// What the Direct default costs, said before anything is sent.
const DIRECT_SEES: &[u8] = b": the far end sees this machine's address";

/// Parse the url and set the request up; a bad url is reported here and
/// there is no job.
pub fn prepare(state: &mut State, args: &[&[u8]]) -> Option<HttpJob> {
    let Some(&raw) = args.first() else {
        state.scrollback.push_error(USAGE);
        return None;
    };
    let Some(url) = parse_url(raw) else {
        state.scrollback.push_error(b"http: bad url");
        return None;
    };
    // A short .anyone name is only as good as the signed list it came from.
    if nonos_route_link::is_short_anyone(&url.host) {
        state.scrollback.push_line(nonos_route_link::SHORT_NOTICE.as_bytes());
    }
    let route = Route::chosen();
    if !matches!(route, Route::Down(_)) {
        let mut line = Vec::from(&b"http: connecting to "[..]);
        line.extend_from_slice(url.host.as_bytes());
        line.push(b' ');
        line.extend_from_slice(route.name().as_bytes());
        // An .anyone service goes through net.anon even on Direct (mixnet/exchange/open.rs).
        if route == Route::Direct && !nonos_route_link::is_anyone(&url.host) {
            line.extend_from_slice(DIRECT_SEES);
        }
        state.scrollback.push_line(&line);
    }
    let request = RequestBuilder::get(&url.host, &url.path).user_agent(AGENT).build();
    let exchange =
        Exchange::new(route, &url.host, url.port, url.secure, rtc_now(), request.bytes, MAX_BODY);
    Some(HttpJob { exchange, shown: Some(Stage::Connecting) })
}
