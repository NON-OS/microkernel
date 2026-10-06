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


//! The handshake from ClientHello to the server's Finished (RFC 5246 7.3).

extern crate alloc;

use alloc::vec::Vec;

use nonos_tls::Io;

use super::alert::description;
use super::constants::*;
use super::error::Tls12Error;
use super::flight;
use super::gather::gather;
use super::hello::client_hello;
use super::keys::{directions, master_secret, verify_data, Share};
use super::messages::header;
use super::prf::Hash;
use super::record::{plain, take, Direction};
use super::stream::Tls12Stream;

/// Handshake with the relay behind `io`, naming `sni`. The session that
/// comes back has proved only that the relay holds its certificate's key;
/// the caller binds that certificate to an identity.
pub fn connect<S: Io>(io: &mut S, sni: &[u8]) -> Result<Tls12Stream, Tls12Error> {
    let hello = client_hello(sni)?;
    io.write_all(&hello.record).map_err(|_| Tls12Error::Io)?;
    let mut transcript = hello.message.clone();
    let mut buf = Vec::new();
    let flight = flight::read(io, &mut buf, &mut transcript, &hello.random)?;
    let suite = flight.hello.suite;
    let hash = Hash::of(suite);

    let share = Share::generate()?;
    let premaster = share.agree(&flight.point)?;
    let mut out = Vec::new();
    if flight.cert_request {
        // An empty certificate_list: this client proves nothing at this layer.
        let empty = [HS_CERTIFICATE, 0, 0, 3, 0, 0, 0];
        transcript.extend_from_slice(&empty);
        out.extend_from_slice(&plain(CT_HANDSHAKE, &empty));
    }
    let mut exchange = header(HS_CLIENT_KEY_EXCHANGE, 66).to_vec();
    exchange.push(65);
    exchange.extend_from_slice(&share.public);
    transcript.extend_from_slice(&exchange);
    out.extend_from_slice(&plain(CT_HANDSHAKE, &exchange));

    let mut randoms = [0u8; 64];
    randoms[..32].copy_from_slice(&hello.random);
    randoms[32..].copy_from_slice(&flight.hello.random);
    let session_hash = if flight.hello.ems { Some(hash.digest(&transcript)?) } else { None };
    let master = master_secret(hash, &premaster.0, session_hash.as_deref(), &randoms)?;
    drop(premaster);
    let (mut client, mut server) = directions(suite, &master.0, &hello.random, &flight.hello.random)?;

    out.extend_from_slice(&plain(CT_CHANGE_CIPHER_SPEC, &[1]));
    let mut finished = header(HS_FINISHED, 12).to_vec();
    finished.extend_from_slice(&verify_data(hash, &master.0, b"client finished", &transcript)?);
    transcript.extend_from_slice(&finished);
    out.extend_from_slice(&client.seal(CT_HANDSHAKE, &finished)?);
    io.write_all(&out).map_err(|_| Tls12Error::Io)?;

    let mut expected = header(HS_FINISHED, 12).to_vec();
    expected.extend_from_slice(&verify_data(hash, &master.0, b"server finished", &transcript)?);
    drop(master);
    server_finished(io, &mut buf, &mut server, &expected)?;
    Ok(Tls12Stream::new(client, server, buf, flight.leaf))
}

/// The server's ChangeCipherSpec, in the clear, then its Finished under the
/// new keys. Nothing else may come between: this client offered no
/// session ticket and asked for no resumption.
fn server_finished<S: Io>(io: &mut S, buf: &mut Vec<u8>, server: &mut Direction, expected: &[u8]) -> Result<(), Tls12Error> {
    loop {
        match take(buf, false)? {
            Some((CT_CHANGE_CIPHER_SPEC, body)) if body == [1] => break,
            Some((CT_ALERT, body)) => return Err(Tls12Error::PeerAlert(description(&body)?)),
            Some(_) => return Err(Tls12Error::Unexpected),
            None => gather(io, buf)?,
        }
    }
    loop {
        match take(buf, false)? {
            Some((CT_HANDSHAKE, body)) => {
                let message = server.open(CT_HANDSHAKE, &body)?;
                return if message.len() == expected.len() && crate::crypto::equal(&message, expected) {
                    Ok(())
                } else {
                    Err(Tls12Error::Finished)
                };
            }
            Some((CT_ALERT, body)) => {
                let alert = server.open(CT_ALERT, &body)?;
                return Err(Tls12Error::PeerAlert(description(&alert)?));
            }
            Some(_) => return Err(Tls12Error::Unexpected),
            None => gather(io, buf)?,
        }
    }
}
