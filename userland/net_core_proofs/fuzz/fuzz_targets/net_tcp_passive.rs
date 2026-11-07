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

//! A frame as net.core's endpoint receives it, through the header decode and,
//! for LISTEN and ACCEPT, the body decode their handlers run: neither reads
//! past the bytes sent, and each answers what a plain reading of the wire
//! format says it should.

#![no_main]

use libfuzzer_sys::fuzz_target;
use net_core_proofs::protocol::tcp::{E_BAD_ADDR, E_BAD_LEN, OP_ACCEPT, OP_LISTEN};
use net_core_proofs::server::parse_req::{parse, HDR_LEN};
use net_core_proofs::server::passive_body::{accept_handle, listen_port};

fn le(b: &[u8]) -> u64 {
    b.iter().rev().fold(0, |n, &x| n << 8 | u64::from(x))
}

/// The body a well formed frame carries, or `None`.
fn framed(data: &[u8]) -> Option<&[u8]> {
    if data.len() < HDR_LEN || le(&data[4..6]) != 1 {
        return None;
    }
    let end = u64::try_from(HDR_LEN).ok()? + le(&data[16..20]);
    (end <= data.len() as u64).then(|| &data[HDR_LEN..end as usize])
}

fn listen_want(body: &[u8]) -> Result<u16, u16> {
    match body.get(..2).map(le) {
        None => Err(E_BAD_LEN),
        Some(0) => Err(E_BAD_ADDR),
        Some(p) => Ok(p as u16),
    }
}

fn accept_want(body: &[u8]) -> Result<u32, u16> {
    body.get(..4).map(|h| le(h) as u32).ok_or(E_BAD_LEN)
}

fuzz_target!(|data: &[u8]| {
    let got = parse(data);
    let Some(body) = framed(data) else {
        assert!(got.is_err());
        return;
    };
    let (req, got_body) = got.unwrap();
    assert_eq!(got_body, body);
    assert_eq!(u64::from(req.op), le(&data[6..8]));
    assert_eq!(u64::from(req.magic), le(&data[0..4]));
    assert_eq!(u64::from(req.request_id), le(&data[12..16]));
    match req.op {
        OP_LISTEN => assert_eq!(listen_port(body), listen_want(body)),
        OP_ACCEPT => assert_eq!(accept_handle(body), accept_want(body)),
        _ => {
            assert_eq!(listen_port(body), listen_want(body));
            assert_eq!(accept_handle(body), accept_want(body));
        }
    }
});
