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
 * Which network a download leaves through. The Nym mixnet when `net.socks5`
 * runs, the Anyone onion network when `net.anon` runs, a direct connection
 * only when neither does. A route that cannot carry a request fails it; it
 * never reverts to a direct one, which would name this machine to the mirror.
 */

use nonos_socket::{lookup, TcpStream};

use super::anon::AnonLink;
use super::link::Link;
use super::socks::SocksLink;

#[derive(Clone, Copy)]
pub enum Route {
    Nym(u32),
    Anon(u32),
    Direct,
}

impl Route {
    pub fn chosen() -> Route {
        match (lookup(b"net.socks5"), lookup(b"net.anon")) {
            (0, 0) => Route::Direct,
            (0, anon) => Route::Anon(anon),
            (socks, _) => Route::Nym(socks),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Route::Nym(_) => "through the Nym mixnet",
            Route::Anon(_) => "through the Anyone onion network",
            Route::Direct => "over a direct connection",
        }
    }

    pub fn connect(self, host: &str, port: u16) -> Result<Link, &'static str> {
        match self {
            Route::Direct => TcpStream::connect(host, port)
                .map(Link::Direct)
                .map_err(|_| "the mirror could not be reached"),
            Route::Nym(p) => SocksLink::connect(p, host, port)
                .map(Link::Nym)
                .map_err(|_| "the Nym mixnet could not reach the mirror"),
            Route::Anon(p) => AnonLink::connect(p, host, port)
                .map(Link::Anon)
                .map_err(|_| "the Anyone onion network could not reach the mirror"),
        }
    }
}
