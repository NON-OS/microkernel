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

//! Hostile bytes against the kernel's manifest and certificate decoders, the
//! first thing the spawn gate reads of a capsule: every truncation of every
//! committed artifact, a byte appended, random damage, and arbitrary input.
//! None may panic. A decoder that read past its input or trusted a length
//! would show here.

use crate::{capsule_manifest, nonos_id_cert};
use std::fs;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn artifacts(suffix: &str) -> Vec<(String, Vec<u8>)> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../nonos-data/trust/capsules");
    let mut out: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(suffix))
        .filter_map(|p| Some((p.display().to_string(), fs::read(&p).ok()?)))
        .collect();
    out.sort();
    out
}

fn manifest_ok(b: &[u8]) -> bool {
    capsule_manifest::decode::decode(b).is_ok()
}

fn cert_ok(b: &[u8]) -> bool {
    nonos_id_cert::decode::decode(b).is_ok()
}

#[test]
fn every_truncation_of_every_manifest_and_certificate_is_refused() {
    for (path, bytes) in artifacts(".manifest.bin") {
        for cut in 0..bytes.len() {
            assert!(!manifest_ok(&bytes[..cut]), "{path} cut at {cut}");
        }
    }
    for (path, bytes) in artifacts(".nonos_id_cert.bin") {
        for cut in 0..bytes.len() {
            assert!(!cert_ok(&bytes[..cut]), "{path} cut at {cut}");
        }
    }
}

#[test]
fn a_byte_after_the_end_is_refused() {
    for (path, mut bytes) in artifacts(".manifest.bin") {
        bytes.push(0);
        assert!(!manifest_ok(&bytes), "{path} with a trailing byte");
    }
    for (path, mut bytes) in artifacts(".nonos_id_cert.bin") {
        bytes.push(0);
        assert!(!cert_ok(&bytes), "{path} with a trailing byte");
    }
}

#[test]
fn random_damage_never_panics() {
    let mut rng = Rng(0x6a09_e667_f3bc_c908);
    for (_, bytes) in artifacts(".manifest.bin").iter().chain(&artifacts(".nonos_id_cert.bin")) {
        for _ in 0..400 {
            let mut bad = bytes.clone();
            for _ in 0..1 + rng.below(6) {
                let at = rng.below(bad.len());
                bad[at] ^= (rng.next() as u8) | 1;
            }
            let _ = manifest_ok(&bad);
            let _ = cert_ok(&bad);
        }
    }
}

#[test]
fn arbitrary_bytes_never_panic() {
    let mut rng = Rng(0xbb67_ae85_84ca_a73b);
    for _ in 0..30_000 {
        let len = rng.below(2048);
        let b: Vec<u8> = (0..len).map(|_| rng.next() as u8).collect();
        let _ = manifest_ok(&b);
        let _ = cert_ok(&b);
    }
}
