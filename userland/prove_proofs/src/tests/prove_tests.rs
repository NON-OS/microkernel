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

//! The one real proof: the pure half's statement and witness, proven by
//! `nonos-device-attest`, verified, written in the output format, read back
//! and verified again from the file alone. Minutes, and about 2.6 GB.

use std::time::Instant;

use nonos_device_attest::{prove, verify, words_of};

use crate::assemble::output::encode;
use crate::assemble::output_read::decode;
use crate::assemble::wipe::wipe;
use crate::fixture::device;

fn peak_kib() -> String {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    status.lines().find(|l| l.starts_with("VmHWM")).unwrap_or("VmHWM unknown").to_string()
}

#[test]
fn an_enrolled_device_proves_and_its_output_verifies() {
    let (st, mut w) = device().assemble().expect("an enrolled device");
    let started = Instant::now();
    let proof = prove(&st, &w, &[0x5C; 64]).expect("a proof");
    let took = started.elapsed();
    wipe(&mut w);
    assert_eq!(verify(&st, &proof), Ok(()));
    let file = encode(&st, &proof).expect("the output");
    let back = decode(&file).expect("the output read back");
    assert_eq!(back.statement, st);
    assert_eq!(verify(&back.statement, &back.proof), Ok(()));
    let mut replayed = st;
    replayed.context = words_of(&[0x43; 32]);
    assert!(verify(&replayed, &back.proof).is_err(), "bound to the verifier's nonce");
    let mut g = file.clone();
    g[156] ^= 0x01;
    let forged = decode(&g).expect("a tag word still below p");
    assert!(verify(&forged.statement, &forged.proof).is_err(), "bound to its tag");
    eprintln!(
        "proved in {took:?}: proof {} bytes, file {} bytes, {}",
        proof.bytes.len(),
        file.len(),
        peak_kib()
    );
}
