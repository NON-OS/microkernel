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


//! The fuzz targets' properties (fuzz/fuzz_targets), checked on every test
//! run over seeded mutations of the proofs' real documents: bit flips,
//! byte swaps, truncations, insertions and splices. cargo-fuzz explores
//! further; this keeps the same assertions in the ordinary proof run, where
//! a regression cannot wait for someone to start the fuzzer.

use std::vec::Vec;

use crate::cell::{body, parse, parse_versions, unpack};
use crate::directory::{consensus, microdesc};
use crate::onion::cells::{introduce_ack, rendezvous2};
use crate::onion::client_auth::{auth_layer, parse_key};
use crate::onion::desc::{decode, intro};
use crate::onion::fetch::{body as hsdir_body, body_within};
use crate::onion::names::{defaults, verify, LIST_MAX};
use crate::onion::pow::params;

const DESC: &[u8] = include_bytes!("../../../anon_ntor_proofs/vectors/onion_descriptor.txt");
const DESC_POW: &[u8] = include_bytes!("../../../anon_ntor_proofs/vectors/onion_descriptor_pow.txt");
const BAD_SIG: &[u8] = include_bytes!("../../../anon_ntor_proofs/vectors/tor_desc_bad_sig.txt");
const CONSENSUS: &[u8] = include_bytes!("../../../anon_ntor_proofs/vectors/consensus-microdesc.txt");
const MICRODESCS: &[u8] = include_bytes!("../../../anon_ntor_proofs/vectors/microdescs.txt");
const BLINDED: [u8; 32] = [
    0x03, 0xa1, 0x07, 0xbf, 0xf3, 0xce, 0x10, 0xbe, 0x1d, 0x70, 0xdd, 0x18, 0xe7, 0x4b, 0xc0, 0x99, 0x67, 0xe4, 0xd6,
    0x30, 0x9b, 0xa5, 0x0d, 0x5f, 0x1d, 0xdc, 0x86, 0x64, 0x12, 0x55, 0x31, 0xb8,
];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }
}

/// One mutation of `seed`, as libFuzzer's simplest mutators make them.
fn mutate(rng: &mut Rng, seed: &[u8], other: &[u8]) -> Vec<u8> {
    let mut d = seed.to_vec();
    for _ in 0..1 + rng.below(4) {
        if d.is_empty() {
            d.push(rng.next() as u8);
        }
        let at = rng.below(d.len());
        match rng.below(6) {
            0 => d[at] ^= 1 << rng.below(8),
            1 => d[at] = rng.next() as u8,
            2 => d.truncate(at),
            3 => d.insert(at, rng.next() as u8),
            4 => {
                let b = rng.below(d.len());
                d.swap(at, b);
            }
            _ => {
                let from = rng.below(other.len());
                let len = rng.below(64).min(other.len() - from);
                d.splice(at..at, other[from..from + len].iter().copied());
            }
        }
    }
    d
}

#[test]
fn descriptors_and_their_layers_survive_hostile_bytes() {
    let mut rng = Rng(0xD5C_0FFE_E123);
    let seeds = [DESC, DESC_POW, BAD_SIG];
    for i in 0..1500 {
        let data = mutate(&mut rng, seeds[i % 3], seeds[(i + 1) % 3]);
        let _ = decode(&data, &BLINDED, &[0x55; 32], 1_790_000_000, |_| Some([7u8; 32]));
        assert!(intro::intro_points(&data, &[0x11; 32], 1_790_000_000).is_empty());
        let _ = params(&data);
        let _ = auth_layer(&data);
        let _ = parse_key(&data);
        let _ = intro::link_specifiers(&data);
    }
}

#[test]
fn no_mutated_list_verifies_under_the_six_services_keys() {
    let mut rng = Rng(0xA11_57ED);
    let seed = b"anyone-hosts-version 1\npublished 2026-10-01 00:00:00\nvalid-until 2026-11-01 00:00:00\n\
lander.anyone iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.anyone\n\
anyone-hosts-signature gadmrvl67444hgzrhsnhzknxaimfnzp6az3wq4d2j7hrf7th34elrrad.anyone\n\
-----BEGIN SIGNATURE-----\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\nAAAAAAAAAAAAAAAAAAAAAA==\n-----END SIGNATURE-----\n";
    let signers = defaults::signers();
    for _ in 0..20000 {
        let data = mutate(&mut rng, seed, seed);
        assert!(verify(&data, &signers, 1_791_028_800).is_err());
        let mut http = b"HTTP/1.0 200 OK\r\n\r\n".to_vec();
        http.extend_from_slice(&data);
        if let Ok(b) = body_within(&http, LIST_MAX) {
            assert!(verify(b, &signers, 1_791_028_800).is_err());
        }
    }
}

#[test]
fn cells_replies_and_directory_documents_survive_hostile_bytes() {
    let mut rng = Rng(0xCE11_5EED);
    let mut relay = std::vec![0u8, 0, 0, 1, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3];
    relay.resize(514, 0);
    let small: [&[u8]; 4] = [&relay, &[0, 0, 7, 0, 4, 0, 4, 0, 5], &[0, 0, 0], &[0u8; 64]];
    for i in 0..20000 {
        let data = mutate(&mut rng, small[i % 4], &relay);
        if let Some((_, used)) = parse(&data) {
            assert!(used <= data.len());
        }
        if let Some((_, used)) = parse_versions(&data) {
            assert!(used <= data.len());
        }
        if data.len() >= 509 {
            let payload: [u8; 509] = data[..509].try_into().unwrap();
            let _ = unpack(&payload);
            let _ = body(&payload);
        }
        let _ = introduce_ack(&data);
        let _ = rendezvous2(&data);
        let _ = hsdir_body(&data);
    }
    for i in 0..60 {
        let data = mutate(&mut rng, if i % 2 == 0 { CONSENSUS } else { MICRODESCS }, MICRODESCS);
        let _ = consensus::parse(&data);
        let _ = microdesc::parse(&data);
        for (start, end) in microdesc::pieces(&data) {
            assert!(start <= end && end <= data.len());
        }
    }
}
