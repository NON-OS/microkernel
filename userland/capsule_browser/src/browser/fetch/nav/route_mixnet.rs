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

use crate::browser::net::{lookup, mixnet};

/// Set the ways the page at `host` may take, before a request of it leaves.
/// True when its network differs from the page before's.
///
/// A reader who chose a private network gets it or gets no page: when its
/// service is not running, each request fails by name (`Routes::way`), and
/// none of them is ever sent direct while the reader believes it is hidden,
/// which is the one disclosure the route exists to prevent. An .anyone
/// address is a page of the Anyone network whatever the choice, refused when
/// net.anon is not running. The choice and the services are read here, once
/// per navigation, so a switch takes effect on the next page and never under
/// a request already in flight.
pub fn route_page(host: &str) -> bool {
    let routes = mixnet::Routes::for_page(
        host,
        mixnet::chosen(),
        lookup(b"net.socks5"),
        lookup(b"net.anon"),
    );
    mixnet::set_routes(routes).page != routes.page
}
