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

//! A record's header, for a handshake whose document and enrolment pass.

use super::attest::doc;
use super::record::{SESSION, START};
use super::{keys, put16, put8};
use nonos_hash::sha512;
use nsh_crypto::attest_challenge;

pub fn record_doc(th_hello: &[u8; 64]) -> Vec<u8> {
    let h = keys::host();
    doc(&attest_challenge(th_hello, &h.ed.public, &h.ml_public))
}

pub fn header(th_hello: &[u8; 64], enrol: &[u8], doc: &[u8]) -> Vec<u8> {
    header_with(SESSION, keys::user().digest(), th_hello, enrol, doc)
}

/// A header naming `session` and `subject`.
pub fn header_with(
    session: [u8; 16],
    subject: [u8; 64],
    th_hello: &[u8; 64],
    enrol: &[u8],
    doc: &[u8],
) -> Vec<u8> {
    let mut out = b"NSHR".to_vec();
    out.extend_from_slice(&[1, 0, 0, 0]);
    out.extend_from_slice(&session);
    out.extend_from_slice(&START.to_le_bytes());
    out.extend_from_slice(th_hello);
    out.extend_from_slice(&keys::host().public_bytes());
    put16(&mut out, enrol);
    out.extend_from_slice(&sha512(doc));
    out.extend_from_slice(&subject);
    put8(&mut out, b"ek");
    put8(&mut out, b"operator");
    out.extend_from_slice(&[0x6A; 64]);
    out.push(2);
    put8(&mut out, b"CoreExec");
    put8(&mut out, b"IPC");
    out
}
