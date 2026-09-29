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

//! Opening a link: TLS, then VERSIONS, CERTS and NETINFO.

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use nonos_tls::stream::connect_unauthenticated;

use crate::cell::parse_versions;
use crate::path::Relay;

use crate::trace;

use super::pump::drain;
use super::session::{Link, LinkError};
use super::socket::Socket;
use super::tls_fault::tls_fault;
use super::versions::{negotiate, offer};
use super::versions_read::read_until_versions;

/// Dial `relay`, prove it is the relay the consensus named, and hand back a link.
///
pub fn open(tcp_port: u32, relay: &Relay, now: u64) -> Result<Link, LinkError> {
    let mut socket =
        Socket::open(tcp_port, relay.address, relay.or_port).ok_or(LinkError::Connect)?;
    trace::say_addr(b"link tcp up", relay.address, relay.or_port);

    /*
     * The SNI is the relay's own address. Relays accept any name, and a fixed
     * string would make every client of this system recognisable by one field
     * of its ClientHello.
     */
    let mut scratch = [0u8; 15];
    let sni = super::sni::write(&mut scratch, relay.address);
    let mut stream = connect_unauthenticated(&mut socket, sni).map_err(tls_fault)?;
    let leaf: Vec<u8> = stream.leaf().ok_or(LinkError::Identity)?.to_vec();
    trace::say(b"link tls up");

    stream.write_all(&mut socket, &offer()).map_err(|_| LinkError::Tls)?;
    let mut partial = read_until_versions(&mut stream, &mut socket)?;
    let (body, used) = parse_versions(&partial).ok_or(LinkError::Protocol)?;
    partial.drain(..used);
    let version = negotiate(&body).ok_or(LinkError::Version)?;
    trace::say_num(b"link version", version as u64);

    let mut link = Link { socket, stream, partial, held: VecDeque::new() };
    drain(&mut link, &leaf, relay, now)?;
    Ok(link)
}
