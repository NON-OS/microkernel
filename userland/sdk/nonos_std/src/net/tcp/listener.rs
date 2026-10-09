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

use super::stream::TcpStream;
use crate::io::{Error, ErrorKind, Result};
use crate::net::addr::{resolve, ToSocketAddrs};
use crate::net::socket::{Socket, KIND_STREAM};
use crate::net::way::{direct_only, LISTEN_OFF_DIRECT};
use crate::net::Route;

pub struct TcpListener {
    inner: Socket,
}

impl TcpListener {
    /// A listener. One on 127.0.0.0/8 takes connections from this machine
    /// whatever network is chosen; any other address only on Direct, since
    /// taking connections from outside names this machine (net/way.rs).
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let (ip, port) = resolve(addr)?;
        if ip[0] != 127 {
            direct_only(Route::chosen(), LISTEN_OFF_DIRECT)
                .map_err(|why| Error::new(ErrorKind::PermissionDenied, why))?;
        }
        let inner = Socket::open(KIND_STREAM)?;
        inner.bind(ip, port)?;
        inner.listen()?;
        Ok(Self { inner })
    }

    pub fn accept(&self) -> Result<TcpStream> {
        let child = self.inner.accept()?;
        Ok(TcpStream::from_socket(child))
    }
}
