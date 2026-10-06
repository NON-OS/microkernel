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


//! The link's proof of who a relay is, over hostile bytes. Every reader
//! runs without panicking, and no CERTS cell binds the captured TLS
//! certificate to an identity it was not signed for: that would be a
//! forged relay.

#![no_main]

use anon_link_proofs::link::bind::bind;
use anon_link_proofs::link::certs::split;
use anon_link_proofs::link::ed_cert::parse;
use anon_link_proofs::link::versions::negotiate;
use anon_link_proofs::vectors::{CAPTURED_AT, LEAF};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Some(entries) = split(data) {
        for entry in entries {
            let _ = parse(entry.body);
        }
    }
    let _ = parse(data);
    let _ = negotiate(data);
    assert!(bind(data, LEAF, &[0x42; 32], CAPTURED_AT).is_err(), "a relay bound to an identity it does not hold");
    assert!(bind(data, data, &[0x42; 32], CAPTURED_AT).is_err());
});
