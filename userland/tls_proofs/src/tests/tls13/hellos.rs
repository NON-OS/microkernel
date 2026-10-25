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


use std::vec::Vec;

/// The hello's legacy session id, cipher suites and extensions.
pub(super) type Hello = (Vec<u8>, Vec<u16>, Vec<(u16, Vec<u8>)>);

pub(super) fn parse_hello(record: &[u8]) -> Hello {
    let body = &record[5 + 4..];
    let sid_len = usize::from(body[34]);
    let sid = body[35..35 + sid_len].to_vec();
    let mut at = 35 + sid_len;
    let n = usize::from(u16::from_be_bytes([body[at], body[at + 1]]));
    let suites = body[at + 2..at + 2 + n].chunks(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
    at += 2 + n;
    at += 1 + usize::from(body[at]);
    let end = at + 2 + usize::from(u16::from_be_bytes([body[at], body[at + 1]]));
    at += 2;
    let mut exts = Vec::new();
    while at < end {
        let kind = u16::from_be_bytes([body[at], body[at + 1]]);
        let len = usize::from(u16::from_be_bytes([body[at + 2], body[at + 3]]));
        exts.push((kind, body[at + 4..at + 4 + len].to_vec()));
        at += 4 + len;
    }
    (sid, suites, exts)
}

/// A TLS 1.2 ServerHello echoing `sid`, naming `suite`, with `exts`.
pub(super) fn tls12_server_hello(sid: &[u8], suite: u16, exts: &[u8]) -> Vec<u8> {
    let mut body = std::vec![3, 3];
    body.extend_from_slice(&[0x5a; 32]);
    body.push(sid.len() as u8);
    body.extend_from_slice(sid);
    body.extend_from_slice(&suite.to_be_bytes());
    body.push(0);
    if !exts.is_empty() {
        body.extend_from_slice(&(exts.len() as u16).to_be_bytes());
        body.extend_from_slice(exts);
    }
    let mut msg = std::vec![2, 0, (body.len() >> 8) as u8, body.len() as u8];
    msg.extend_from_slice(&body);
    let mut record = std::vec![22, 3, 3, (msg.len() >> 8) as u8, msg.len() as u8];
    record.extend_from_slice(&msg);
    record
}
