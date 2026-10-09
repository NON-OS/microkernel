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


//! A descriptor and each layer inside it. The whole document is checked
//! for a signature first, which random bytes never carry, so the layers'
//! own readers are driven directly too: the introduction points, the
//! proof-of-work line, the client authorization block and key lines. None
//! may panic or read past its input, and no introduction point may come
//! back: each needs certificates signed by a key the fuzzer does not hold.

#![no_main]

use anon_onion_proofs::onion::client_auth::{auth_layer, parse_key};
use anon_onion_proofs::onion::desc::{decode, intro};
use anon_onion_proofs::onion::pow::params;
use libfuzzer_sys::fuzz_target;

/// The fixture's blinded key and subcredential (anon_ntor_proofs vectors).
const BLINDED: [u8; 32] = [
    0x03, 0xa1, 0x07, 0xbf, 0xf3, 0xce, 0x10, 0xbe, 0x1d, 0x70, 0xdd, 0x18, 0xe7, 0x4b, 0xc0, 0x99, 0x67, 0xe4, 0xd6,
    0x30, 0x9b, 0xa5, 0x0d, 0x5f, 0x1d, 0xdc, 0x86, 0x64, 0x12, 0x55, 0x31, 0xb8,
];
const NOW: u64 = 1_790_000_000;

fuzz_target!(|data: &[u8]| {
    let _ = decode(data, &BLINDED, &[0x55; 32], NOW, |_| Some([7u8; 32]));
    assert!(intro::intro_points(data, &[0x11; 32], NOW).is_empty(), "an introduction point without a valid certificate");
    let _ = params(data);
    let _ = auth_layer(data);
    let _ = parse_key(data);
    let _ = intro::link_specifiers(data);
});
