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

//! Telling a server that only speaks TLS 1.2 from one that is broken.

/// Handshake record content type, RFC 8446 section 5.1.
const HANDSHAKE: u8 = 22;
/// ServerHello handshake message type.
const SERVER_HELLO: u8 = 2;
/// The supported_versions extension.
const SUPPORTED_VERSIONS: u16 = 43;
const TLS12: u16 = 0x0303;
/// Alerts a 1.2-only server sends in the clear to a hello it cannot meet.
const HANDSHAKE_FAILURE: u8 = 40;
const PROTOCOL_VERSION: u8 = 70;

/*
 * The handshake has already refused the ServerHello by the time this runs;
 * this reads it again only to say why. A TLS 1.3 ServerHello always carries
 * supported_versions naming 1.3 (RFC 8446 section 4.2.1). A server that
 * picked 1.2 or lower either leaves the extension out and puts its version
 * in legacy_version, or names the lower version in the extension.
 */
/// Whether the first record of `flight` is a ServerHello choosing TLS 1.2
/// or older.
pub fn selects_tls12(flight: &[u8]) -> bool {
    hello_version(flight).is_some_and(|v| v <= TLS12)
}

/// Whether a plaintext alert received before any ServerHello is the one a
/// server that does not speak TLS 1.3 answers a 1.3-only hello with.
pub fn tls12_alert(description: u8) -> bool {
    description == PROTOCOL_VERSION || description == HANDSHAKE_FAILURE
}

fn hello_version(flight: &[u8]) -> Option<u16> {
    let head = flight.get(..5)?;
    if head[0] != HANDSHAKE {
        return None;
    }
    let record_len = u16::from_be_bytes([head[3], head[4]]) as usize;
    let record = flight.get(5..5 + record_len)?;
    if *record.first()? != SERVER_HELLO {
        return None;
    }
    let len = u32::from_be_bytes([0, *record.get(1)?, *record.get(2)?, *record.get(3)?]) as usize;
    let body = record.get(4..4 + len)?;
    let legacy = u16_at(body, 0)?;
    /* version, random, then the echoed session id. */
    let sid = *body.get(34)? as usize;
    /* cipher suite and compression method. */
    let ext_at = 35 + sid + 3;
    if body.len() == ext_at {
        return Some(legacy);
    }
    let ext_len = u16_at(body, ext_at)? as usize;
    let mut exts = body.get(ext_at + 2..ext_at + 2 + ext_len)?;
    while exts.len() >= 4 {
        let kind = u16_at(exts, 0)?;
        let n = u16_at(exts, 2)? as usize;
        let value = exts.get(4..4 + n)?;
        if kind == SUPPORTED_VERSIONS {
            return u16_at(value, 0);
        }
        exts = &exts[4 + n..];
    }
    Some(legacy)
}

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*b.get(at)?, *b.get(at + 1)?]))
}
