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
 * An opening on a named stream of the proxy, for a caller that holds
 * several streams through one proxy at once (`frame_on`): the Linux
 * personality opens one for each guest connection it carries. The rest is
 * `RouteOpening::begin`, unchanged: the reset, the greeting, the CONNECT
 * naming the host unresolved, and the retries while a network that runs
 * has not connected yet, all under OPEN_MS.
 */

use crate::frame_on::STREAMS_PER_CALLER;
use crate::ipc::Ipc;
use crate::opening::Opening;
use crate::pick::Route;
use crate::refusal::Proxy;
use crate::stream::RouteOpening;

const NOT_PROXIED: &str = "a direct connection is not opened through a proxy";

const NO_STREAM: &str = "a named stream is numbered from 1 to 7";

impl RouteOpening {
    /*
     * As `begin`, on stream `stream` of the proxy `route` names, 1 to 7.
     * An .anyone host is the caller's to route: through Anyone it is
     * reported as any other host, and through Nym it is never reached.
     */
    pub fn begin_on(
        route: Route,
        host: &str,
        port: u16,
        slice_ms: u64,
        stream: u32,
    ) -> Result<RouteOpening, &'static str> {
        let (port_of, proxy) = match route {
            Route::Nym(p) => (p, Proxy::Nym),
            Route::Anon(p) => (p, Proxy::Anyone),
            Route::Direct => return Err(NOT_PROXIED),
            Route::Down(why) => return Err(why),
        };
        if stream == 0 || stream >= STREAMS_PER_CALLER {
            return Err(NO_STREAM);
        }
        Opening::begin(Ipc::on(port_of, stream), proxy, host, port, slice_ms).map(RouteOpening)
    }
}
