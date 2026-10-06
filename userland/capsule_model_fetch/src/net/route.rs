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
 * Which network a download leaves through: the system's default network,
 * which setup asks for and Settings changes, as `pick` decides it
 * (`Route::chosen`, in nonos_route_link). A default whose network is not
 * running is no route, and a route that cannot carry a request fails it;
 * neither reverts to another network, which would name this machine to the
 * mirror. A policy store that cannot be asked leaves the default, the
 * mixnet, and never a direct connection.
 */

use nonos_socket::TcpStream;

use super::anon::AnonLink;
use super::link::Link;
use super::socks::{SocksLink, NET_UNREACHABLE};
use super::Route;
use crate::http::Fault;

/*
 * A connection to `host`, or why not. Through Nym, net.socks5 having no
 * session at all is told apart from an exit that did not answer, so a
 * network not reachable from this machine is never said as an exit that
 * went silent, nor the other way round.
 */
pub fn connect(route: Route, host: &str, port: u16) -> Result<Link, Fault> {
    match route {
        Route::Direct => TcpStream::connect(host, port)
            .map(Link::Direct)
            .map_err(|_| Fault::Net("the mirror could not be reached")),
        Route::Nym(p) => SocksLink::connect(p, host, port).map(Link::Nym).map_err(|code| {
            match code {
                NET_UNREACHABLE => Fault::NoSession("the Nym mixnet has no session to reach it through"),
                _ => Fault::Net("no Nym exit answered"),
            }
        }),
        Route::Anon(p) => AnonLink::connect(p, host, port)
            .map(Link::Anon)
            .map_err(|_| Fault::Net("no Anyone circuit reached the mirror")),
        Route::Down(why) => Err(Fault::Net(why)),
    }
}
