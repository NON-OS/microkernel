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

use alloc::vec::Vec;
use nonos_tls::stream::connect_unauthenticated;
use nonos_tls::SessionError;
use nonos_libc::crypto_random;

use crate::cell::parse_versions;
use crate::path::Relay;

use crate::trace;

use super::fallback::refuses_tls13;
use super::pump::drain;
use super::session::{Link, LinkError, Tls};
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
     * Relays accept any name, and a fixed string would make every client of
     * this system recognisable by one field of its ClientHello. Built with
     * `random-sni` the name is tor's own shape, drawn fresh per link
     * (sni_random.rs); without it, the relay's address, an IP literal RFC
     * 6066 does not allow there. On by default, signed off as a separate step on
     * 2026-11-02.
     */
    let mut quad = [0u8; 15];
    let mut name = [0u8; super::sni_random::NAME_MAX];
    let sni = if cfg!(feature = "random-sni") {
        let mut seed = [0u8; 21];
        if crypto_random(seed.as_mut_ptr(), seed.len()) != seed.len() as i64 {
            return Err(LinkError::Connect);
        }
        super::sni_random::random_name(&mut name, &seed)
    } else {
        super::sni::write(&mut quad, relay.address)
    };
    let (mut stream, leaf) = match connect_unauthenticated(&mut socket, sni) {
        Ok(s) => {
            let leaf: Vec<u8> = s.leaf().ok_or(LinkError::Identity)?.to_vec();
            trace::say(b"link tls up");
            (Tls::V13(s), leaf)
        }
        /* The rule, and why it cannot downgrade a relay that speaks 1.3, is in
         * fallback.rs. */
        Err(SessionError::PeerAlert(description)) if refuses_tls13(description) => {
            let _ = tls_fault(SessionError::PeerAlert(description));
            trace::say(b"link relay refused tls 1.3, trying tls 1.2");
            drop(socket);
            socket = Socket::open(tcp_port, relay.address, relay.or_port).ok_or(LinkError::Connect)?;
            let s = super::tls12::connect(&mut socket, sni).map_err(|why| {
                trace::say(why.said());
                if let super::tls12::Tls12Error::PeerAlert(d) = why {
                    trace::say_num(b"guard tls12: peer alert", d as u64);
                }
                LinkError::Tls
            })?;
            trace::say(b"link tls 1.2 up");
            let leaf = s.leaf().to_vec();
            (Tls::V12(s), leaf)
        }
        Err(cause) => return Err(tls_fault(cause)),
    };

    stream.write_all(&mut socket, &offer()).map_err(|_| LinkError::Tls)?;
    let mut partial = read_until_versions(&mut stream, &mut socket)?;
    let (body, used) = parse_versions(&partial).ok_or(LinkError::Protocol)?;
    partial.drain(..used);
    let version = negotiate(&body).ok_or(LinkError::Version)?;
    trace::say_num(b"link version", version as u64);

    let mut link = Link { socket, stream, partial };
    drain(&mut link, &leaf, relay, now)?;
    Ok(link)
}
