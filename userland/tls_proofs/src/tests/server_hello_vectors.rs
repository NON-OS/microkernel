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

//! Building a ServerHello to hand the parser.

pub(crate) const X25519: u16 = 0x001d;
pub(crate) const AES128: u16 = 0x1301;

pub(crate) fn hello(suite: u16, sid: &[u8], exts: &[u8], random: [u8; 32]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&[0x03, 0x03]);
    body.extend_from_slice(&random);
    body.push(sid.len() as u8);
    body.extend_from_slice(sid);
    body.extend_from_slice(&suite.to_be_bytes());
    body.push(0); // compression
    body.extend_from_slice(&(exts.len() as u16).to_be_bytes());
    body.extend_from_slice(exts);

    let mut msg = vec![2u8];
    msg.extend_from_slice(&[(body.len() >> 16) as u8, (body.len() >> 8) as u8, body.len() as u8]);
    msg.extend_from_slice(&body);
    msg
}

pub(crate) fn ext(kind: u16, body: &[u8]) -> Vec<u8> {
    let mut out = kind.to_be_bytes().to_vec();
    out.extend_from_slice(&(body.len() as u16).to_be_bytes());
    out.extend_from_slice(body);
    out
}

/// supported_versions saying TLS 1.3, then a key share carrying a 32 byte key.
pub(crate) fn good_exts(key: [u8; 32]) -> Vec<u8> {
    let mut share = X25519.to_be_bytes().to_vec();
    share.extend_from_slice(&32u16.to_be_bytes());
    share.extend_from_slice(&key);
    let mut out = ext(43, &[0x03, 0x04]);
    out.extend_from_slice(&ext(51, &share));
    out
}
