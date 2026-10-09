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

use alloc::format;

use super::anyone::AnyoneStream;
use crate::io::{Error, ErrorKind, Read, Result, Write};
use crate::net::addr::{Target, ToSocketAddrs};
use crate::net::socket::{Socket, KIND_MIXNET, KIND_STREAM};
use crate::net::way::{way, Way};
use crate::net::Route;

pub struct TcpStream {
    inner: Inner,
}

enum Inner {
    Socket(Socket),
    Anyone(AnyoneStream),
}

impl TcpStream {
    /// Connect over the network the person chose (net/way.rs): a direct
    /// socket on Direct, the Nym mixnet or the Anyone network otherwise, and
    /// a name is looked up on this machine only on Direct.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let target = addr.target()?;
        let inner = match way(Route::chosen()) {
            Way::Direct => {
                let (ip, port) = match target {
                    Target::Addr(ip, port) => (ip, port),
                    Target::Name(host, port) => {
                        (crate::net::dns::resolve_host(&host)?.octets(), port)
                    }
                };
                let socket = Socket::open(KIND_STREAM)?;
                socket.connect(ip, port)?;
                Inner::Socket(socket)
            }
            Way::Mixnet => {
                let socket = Socket::open(KIND_MIXNET)?;
                match target {
                    Target::Addr(ip, port) => socket.connect(ip, port)?,
                    Target::Name(host, port) => socket.connect_host(&host, port)?,
                }
                Inner::Socket(socket)
            }
            Way::Anyone(route) => {
                let (host, port) = match target {
                    Target::Addr([a, b, c, d], port) => (format!("{a}.{b}.{c}.{d}"), port),
                    Target::Name(host, port) => (host, port),
                };
                Inner::Anyone(AnyoneStream::open(route, &host, port)?)
            }
            Way::Down(why) => return Err(Error::new(ErrorKind::Other, why)),
        };
        Ok(Self { inner })
    }

    pub(crate) fn from_socket(inner: Socket) -> Self {
        Self { inner: Inner::Socket(inner) }
    }
}

impl Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match &mut self.inner {
            Inner::Socket(s) => s.recv(buf),
            Inner::Anyone(s) => s.read(buf),
        }
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match &mut self.inner {
            Inner::Socket(s) => s.send(buf),
            Inner::Anyone(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
