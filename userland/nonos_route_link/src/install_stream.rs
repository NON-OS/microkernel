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
 * A stream for an install's download: the route installs take
 * (`Route::for_installs`, Anyone or no route at all), with the install
 * mark in its greeting (install.rs) so net.anon gives it the circuit it
 * keeps warm for installs. Refused as `connect` refuses, and never
 * answered by trying another way.
 */

use crate::ipc::Ipc;
use crate::pick::{Route, ANYONE_INSTALLS_DOWN};
use crate::stream::{for_host, RouteStream};
use crate::tunnel::Tunnel;

impl RouteStream {
    pub fn connect_install(host: &str, port: u16) -> Result<RouteStream, &'static str> {
        #[cfg(feature = "devnet")]
        let host = crate::devnet::dial_at(host);
        match for_host(Route::for_installs(), host) {
            (Route::Anon(p), anon) => {
                Tunnel::open_as(Ipc::new(p), anon, host, port, true).map(RouteStream::Proxied)
            }
            (Route::Down(why), _) => Err(why),
            /* The install route is Anyone or none, and for_host keeps it so. */
            _ => Err(ANYONE_INSTALLS_DOWN),
        }
    }
}
