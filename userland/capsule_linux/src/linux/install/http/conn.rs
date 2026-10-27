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

//! A connection of a window of GETs (get/window.rs): a route stream on the
//! network the person chose, reused while its replies leave it fit, or in a
//! dev-mirror build only a socket of this capsule's own to a mirror on the
//! local network, one per GET (direct.rs says why). Reads: conn_io.rs.

use nonos_route_link::{Route, RouteStream};

use super::stream::Stream;
#[cfg(feature = "dev-mirror")]
use crate::linux::net::{raw, raw_io, route::is_local};

pub enum Conn {
    Route(Stream),
    #[cfg(feature = "dev-mirror")]
    Raw(u32),
}

impl Conn {
    /// A connection to `at`:`port`, as get.rs would make one, or why not.
    pub fn open(at: &str, port: u16) -> Result<Conn, &'static str> {
        /* Nothing reaches a network from a family that holds a model. */
        if crate::linux::file::models::held() {
            return Err("this family holds a model");
        }
        #[cfg(feature = "dev-mirror")]
        if is_local(at) {
            return Conn::Raw(raw::open_stream_to(at).ok_or("no socket")?).connected(at, port);
        }
        RouteStream::connect(Route::for_installs(), at, port).map(|s| Conn::Route(Stream(s)))
    }

    /// A route stream may carry the next GET; a raw socket never does.
    pub fn keepable(&self) -> bool {
        matches!(self, Conn::Route(_)) && !self.ended()
    }

    /// The far end finished. A raw socket does not say: an empty read on it
    /// is "nothing yet" (raw.rs, recv_now).
    pub fn ended(&self) -> bool {
        matches!(self, Conn::Route(s) if s.0.ended())
    }

    /// A raw socket connected, or closed by its drop.
    #[cfg(feature = "dev-mirror")]
    fn connected(self, at: &str, port: u16) -> Result<Conn, &'static str> {
        let Conn::Raw(h) = self else { return Ok(self) };
        raw::connect_host(h, at, port).map(|()| self).ok_or("it did not connect")
    }
}

#[cfg(feature = "dev-mirror")]
impl Drop for Conn {
    fn drop(&mut self) {
        #[cfg(feature = "dev-mirror")]
        if let Conn::Raw(h) = self {
            raw_io::close(*h);
        }
    }
}
