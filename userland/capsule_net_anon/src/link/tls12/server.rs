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


//! The server's messages, each checked against exactly what was offered.

extern crate alloc;

use alloc::vec::Vec;

use super::constants::*;
use super::error::Tls12Error;

/// A cursor over a message body that refuses to read past its end.
struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], Tls12Error> {
        let end = self.at.checked_add(n).ok_or(Tls12Error::Malformed)?;
        let out = self.bytes.get(self.at..end).ok_or(Tls12Error::Malformed)?;
        self.at = end;
        Ok(out)
    }
    fn u8(&mut self) -> Result<u8, Tls12Error> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> Result<u16, Tls12Error> {
        let b = self.take(2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]))
    }
    fn u24(&mut self) -> Result<usize, Tls12Error> {
        let b = self.take(3)?;
        Ok(usize::from(b[0]) << 16 | usize::from(b[1]) << 8 | usize::from(b[2]))
    }
    fn done(&self) -> bool {
        self.at == self.bytes.len()
    }
}

pub struct ServerHello {
    pub random: [u8; 32],
    pub suite: u16,
    /// The server agreed to the extended master secret.
    pub ems: bool,
}

/// ServerHello (RFC 5246 7.4.1.3). The server must pick TLS 1.2, one of our
/// suites and no compression, and may only answer extensions we sent.
pub fn server_hello(body: &[u8]) -> Result<ServerHello, Tls12Error> {
    let mut c = Cursor::new(body);
    if c.u16()? != VERSION {
        return Err(Tls12Error::Unoffered);
    }
    let mut random = [0u8; 32];
    random.copy_from_slice(c.take(32)?);
    if &random[24..] == DOWNGRADE_TLS12 {
        return Err(Tls12Error::Downgrade);
    }
    let session = usize::from(c.u8()?);
    if session > 32 {
        return Err(Tls12Error::Malformed);
    }
    c.take(session)?;
    let suite = c.u16()?;
    if !SUITES.contains(&suite) {
        return Err(Tls12Error::Unoffered);
    }
    if c.u8()? != 0 {
        return Err(Tls12Error::Unoffered);
    }
    let mut ems = false;
    if !c.done() {
        let len = usize::from(c.u16()?);
        let mut e = Cursor::new(c.take(len)?);
        let mut seen: Vec<u16> = Vec::new();
        while !e.done() {
            let kind = e.u16()?;
            let len = usize::from(e.u16()?);
            let data = e.take(len)?;
            if seen.contains(&kind) {
                return Err(Tls12Error::Malformed);
            }
            seen.push(kind);
            match kind {
                EXT_EXTENDED_MASTER_SECRET if data.is_empty() => ems = true,
                // RFC 5746 3.4: on an initial handshake the server's
                // renegotiated_connection is empty.
                EXT_RENEGOTIATION_INFO if data == [0] => {}
                EXT_EC_POINT_FORMATS if point_formats_ok(data) => {}
                EXT_SERVER_NAME if data.is_empty() => {}
                EXT_EXTENDED_MASTER_SECRET | EXT_RENEGOTIATION_INFO | EXT_EC_POINT_FORMATS | EXT_SERVER_NAME => {
                    return Err(Tls12Error::Malformed)
                }
                _ => return Err(Tls12Error::Unoffered),
            }
        }
    }
    if !c.done() {
        return Err(Tls12Error::Malformed);
    }
    Ok(ServerHello { random, suite, ems })
}

/// The server's point formats must include the uncompressed one we send.
fn point_formats_ok(data: &[u8]) -> bool {
    match data.split_first() {
        Some((&n, list)) => usize::from(n) == list.len() && list.contains(&POINT_UNCOMPRESSED),
        None => false,
    }
}

/// Certificate (RFC 5246 7.4.2): the leaf, DER. Further certificates are
/// read for their bounds and not used; the CERTS cell is what vouches for
/// the leaf.
pub fn certificate(body: &[u8]) -> Result<Vec<u8>, Tls12Error> {
    let mut c = Cursor::new(body);
    let total = c.u24()?;
    let mut list = Cursor::new(c.take(total)?);
    if !c.done() {
        return Err(Tls12Error::Malformed);
    }
    let mut leaf = None;
    while !list.done() {
        let len = list.u24()?;
        let cert = list.take(len)?;
        if cert.is_empty() {
            return Err(Tls12Error::Malformed);
        }
        if leaf.is_none() {
            leaf = Some(cert.to_vec());
        }
    }
    leaf.ok_or(Tls12Error::Malformed)
}

pub struct KeyExchange {
    /// The server's P-256 point, uncompressed.
    pub point: [u8; 65],
    /// curve_type through the point: what the signature covers after the
    /// two randoms.
    pub params: Vec<u8>,
    pub scheme: u16,
    pub signature: Vec<u8>,
}

/// ServerKeyExchange for ECDHE (RFC 8422 5.4, RFC 5246 7.4.3): a named
/// P-256 point and a signature under one of the schemes we listed.
pub fn key_exchange(body: &[u8]) -> Result<KeyExchange, Tls12Error> {
    let mut c = Cursor::new(body);
    if c.u8()? != CURVE_TYPE_NAMED || c.u16()? != GROUP_P256 {
        return Err(Tls12Error::Unoffered);
    }
    if c.u8()? != 65 {
        return Err(Tls12Error::Malformed);
    }
    let raw = c.take(65)?;
    if raw[0] != 0x04 {
        return Err(Tls12Error::Malformed);
    }
    let mut point = [0u8; 65];
    point.copy_from_slice(raw);
    let params = body[..c.at].to_vec();
    let scheme = c.u16()?;
    if !SIGNATURES.contains(&scheme) {
        return Err(Tls12Error::Unoffered);
    }
    let len = usize::from(c.u16()?);
    let signature = c.take(len)?.to_vec();
    if !c.done() || signature.is_empty() {
        return Err(Tls12Error::Malformed);
    }
    Ok(KeyExchange { point, params, scheme, signature })
}

/// CertificateRequest (RFC 5246 7.4.4). Every Tor-lineage relay sends one;
/// this client answers it with an empty Certificate, which the relay
/// accepts, since a client proves nothing at this layer. It is read only
/// for its bounds.
pub fn certificate_request(body: &[u8]) -> Result<(), Tls12Error> {
    let mut c = Cursor::new(body);
    let types = usize::from(c.u8()?);
    c.take(types)?;
    let sigs = usize::from(c.u16()?);
    if sigs % 2 != 0 {
        return Err(Tls12Error::Malformed);
    }
    c.take(sigs)?;
    let names = usize::from(c.u16()?);
    c.take(names)?;
    if c.done() {
        Ok(())
    } else {
        Err(Tls12Error::Malformed)
    }
}
