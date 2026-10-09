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


//! What relays and authorities send: cells off the link, relay messages
//! inside them, the replies to an introduction and a rendezvous, an HSDir's
//! HTTP answer, and consensus and microdescriptor documents. None may
//! panic, and a cell parse may never claim more bytes than it was given.

#![no_main]

use anon_onion_proofs::cell::{body, parse, parse_versions, unpack};
use anon_onion_proofs::directory::{consensus, microdesc};
use anon_onion_proofs::onion::cells::{introduce_ack, rendezvous2};
use anon_onion_proofs::onion::fetch::body as hsdir_body;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Some((_, used)) = parse(data) {
        assert!(used <= data.len());
    }
    if let Some((_, used)) = parse_versions(data) {
        assert!(used <= data.len());
    }
    if data.len() >= 509 {
        let mut payload = [0u8; 509];
        payload.copy_from_slice(&data[..509]);
        let _ = unpack(&payload);
        let _ = body(&payload);
    }
    let _ = introduce_ack(data);
    let _ = rendezvous2(data);
    let _ = hsdir_body(data);
    let _ = consensus::parse(data);
    let _ = microdesc::parse(data);
    for (start, end) in microdesc::pieces(data) {
        assert!(start <= end && end <= data.len());
    }
});
