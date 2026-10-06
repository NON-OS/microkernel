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

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::FromStr;

use super::any::SocketAddr;
use super::ip::Ipv4Addr;
use super::v4::SocketAddrV4;
use crate::io::{Error, ErrorKind, Result};

/// Where a connection is to go: an address, or a name and a port not yet
/// looked up. A name is looked up on this machine only on the Direct
/// network; on Nym or Anyone the exit resolves it.
#[doc(hidden)]
pub enum Target {
    Addr([u8; 4], u16),
    Name(String, u16),
}

pub trait ToSocketAddrs {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>>;

    /// The target without looking a name up. Every address type is one
    /// already; a name keeps its name.
    #[doc(hidden)]
    fn target(&self) -> Result<Target> {
        let (ip, port) = resolve_addrs(self.to_socket_addrs()?)?;
        Ok(Target::Addr(ip, port))
    }
}

impl ToSocketAddrs for SocketAddr {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        Ok(vec![*self])
    }
}

impl ToSocketAddrs for SocketAddrV4 {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        Ok(vec![SocketAddr::V4(*self)])
    }
}

impl ToSocketAddrs for str {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        match self.target()? {
            Target::Addr(ip, port) => Ok(vec![SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]),
                port,
            ))]),
            Target::Name(host, port) => {
                let ip = crate::net::dns::resolve_host(&host)?;
                Ok(vec![SocketAddr::V4(SocketAddrV4::new(ip, port))])
            }
        }
    }

    fn target(&self) -> Result<Target> {
        if let Ok(v4) = SocketAddrV4::from_str(self) {
            return Ok(Target::Addr(v4.ip().octets(), v4.port()));
        }
        let (host, port) = self
            .rsplit_once(':')
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "missing port"))?;
        let port: u16 = port.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "bad port"))?;
        if host.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "missing host"));
        }
        Ok(Target::Name(String::from(host), port))
    }
}

impl ToSocketAddrs for &str {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        (**self).to_socket_addrs()
    }

    fn target(&self) -> Result<Target> {
        (**self).target()
    }
}

pub(crate) fn resolve<A: ToSocketAddrs>(addr: A) -> Result<([u8; 4], u16)> {
    resolve_addrs(addr.to_socket_addrs()?)
}

fn resolve_addrs(addrs: Vec<SocketAddr>) -> Result<([u8; 4], u16)> {
    let first =
        addrs.into_iter().next().ok_or_else(|| Error::new(ErrorKind::InvalidInput, "no address"))?;
    Ok(first.v4_parts())
}
