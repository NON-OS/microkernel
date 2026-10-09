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

//! A server's certificate bytes: the DER walker and the certificate field
//! readers never panic and never point outside what they were given, and no
//! byte string chains to a trusted root, since that would be a forgery.

#![no_main]

use libfuzzer_sys::fuzz_target;
use tls_proofs::{cert_ext, cert_spki, cert_tbs, chain_walk, der_tlv};

/* id-ce-subjectAltName, basicConstraints, keyUsage */
const OIDS: [&[u8]; 3] = [&[0x55, 0x1d, 0x11], &[0x55, 0x1d, 0x13], &[0x55, 0x1d, 0x0f]];

fn inside(outer: &[u8], inner: &[u8]) -> bool {
    let (o, i) = (outer.as_ptr() as usize, inner.as_ptr() as usize);
    i >= o && i + inner.len() <= o + outer.len()
}

fuzz_target!(|data: &[u8]| {
    let mut off = 0;
    while let Some((_, val, end)) = der_tlv::der_tlv(data, off) {
        assert!(off + 2 <= val && val <= end && end <= data.len());
        off = val;
    }
    if let Some(tbs) = cert_tbs::cert_tbs(data) {
        assert!(inside(data, tbs));
    }
    if let Some(spki) = cert_spki::cert_spki(data) {
        assert!(inside(data, spki));
    }
    for oid in OIDS {
        if let Some(v) = cert_ext::extension_value(data, oid) {
            assert!(inside(data, v));
        }
    }
    /* The gateway seed's host and month: its chain is real, its root trusted by no one. */
    let now = 20261001000000;
    assert!(!chain_walk::verify_chain(data, b"nonos.software", now), "a chain was forged");
});
