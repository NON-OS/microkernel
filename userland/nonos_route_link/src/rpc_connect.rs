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

//! A chain RPC's connection (P5): as any other, except that a `devnet` build
//! dials the local test chain's one name at the development host. The name
//! itself still goes to TLS, so its certificate is checked against it.

use crate::pick::Route;
use crate::stream::RouteStream;

impl RouteStream {
    pub fn connect_rpc(route: Route, host: &str, port: u16) -> Result<RouteStream, &'static str> {
        #[cfg(feature = "devnet")]
        let host = crate::devnet::dial_at(host);
        RouteStream::connect(route, host, port)
    }
}
