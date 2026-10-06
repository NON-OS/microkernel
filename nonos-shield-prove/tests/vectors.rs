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

//! The four pinned production vectors, proved again here byte for byte: the
//! proof, its format 7 form, and both verifying. The production pool's
//! verifier accepts these four, so a prover that reproduces them proves what
//! the pool accepts. STARKS_SPEC names the vectors' folder, the `spec`
//! directory of the STARKs checkout the flake pins. Four proofs, so it is
//! run on purpose: cargo test --release --features parallel -- --ignored

use std::fs;

use nonos_shield_prove::{prove, prove_cached, share, verify, verify_shared, Progress};

fn unhex(text: &str) -> Vec<u8> {
    let t = text.trim().trim_start_matches("0x");
    (0..t.len()).step_by(2).map(|i| u8::from_str_radix(&t[i..i + 2], 16).unwrap()).collect()
}

#[test]
#[ignore = "proves four times; needs STARKS_SPEC"]
fn the_four_pinned_production_vectors_prove_byte_for_byte() {
    let spec = std::env::var("STARKS_SPEC").expect("STARKS_SPEC");
    let dir = format!("{spec}/wallet-vectors-not-before");
    let mut cache = Vec::new();
    for name in ["transfer-eth", "withdraw-eth", "transfer-nox", "withdraw-nox"] {
        let text = |f: &str| fs::read_to_string(format!("{dir}/{name}/{f}")).unwrap();
        let entropy = unhex(&text("entropy.hex"));
        let none = Progress::default();
        let proof = if cache.is_empty() {
            let (p, c) = prove(&text("request.json"), &text("seed.json"), &entropy, &none).unwrap();
            cache = c;
            p
        } else {
            prove_cached(&text("request.json"), &text("seed.json"), &entropy, &cache, &none).unwrap()
        };
        assert_eq!(proof.bytes, fs::read(format!("{dir}/{name}/proof.bin")).unwrap(), "{name}");
        verify(&proof.bytes, &proof.publics).unwrap();
        let shared = share(&proof.bytes, &proof.publics).unwrap();
        assert_eq!(shared, fs::read(format!("{dir}/{name}/proof-format7.bin")).unwrap(), "{name}");
        verify_shared(&shared, &proof.publics).unwrap();
        println!("{name}: byte for byte, {} bytes shared", shared.len());
    }
}
