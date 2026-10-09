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

//! The boot-root record against the record the release tool signed: the input
//! is a list of edits to it. Whatever is admitted names the signed root and
//! epoch; no edit makes another root or epoch verify.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_boot_measure::record::check;
use p256::ecdsa::signature::hazmat::PrehashVerifier;
use p256::ecdsa::{Signature, VerifyingKey};

const RECORD: &[u8] = include_bytes!("../../src/tests/fixture/boot_root.approval");
const PUB: &[u8; 64] = include_bytes!("../../src/tests/fixture/boot_root.pub");

fn release(d: &[u8; 32], r: &[u8; 32], s: &[u8; 32]) -> bool {
    let sec1 = [&[0x04u8][..], PUB].concat();
    let (Ok(key), Ok(sig)) =
        (VerifyingKey::from_sec1_bytes(&sec1), Signature::from_scalars(*r, *s))
    else {
        return false;
    };
    key.verify_prehash(d, &sig).is_ok()
}

fuzz_target!(|data: &[u8]| {
    let mut b = RECORD.to_vec();
    for e in data.chunks_exact(2) {
        match b.get_mut(usize::from(e[0])) {
            Some(x) => *x ^= e[1].max(1),
            None => b.push(e[1]),
        }
    }
    let floor = data.first().map_or(0, |&f| u64::from(f) % 9);
    if let Ok(rec) = check(&b, floor, release) {
        assert_eq!((&rec.root[..], rec.epoch), (&RECORD[..32], 7));
    }
});
