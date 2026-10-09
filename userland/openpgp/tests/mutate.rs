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

//! Seeded mutation fuzzing of the packet, key and signature parsers, not
//! coverage-guided (no libFuzzer is vendored). Every damaged input must
//! return, in a debug build, without a panic or an overflow.

#[path = "support/bignum.rs"]
mod bignum;
#[path = "support/host.rs"]
mod host;
#[path = "support/modpow.rs"]
mod modpow;

use host::Host;
use nonos_openpgp::{keys, verify};

const ROUNDS: usize = 3000;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn damage(&mut self, v: &mut Vec<u8>) {
        let at = (self.next() % v.len().max(1) as u64) as usize;
        match self.next() % 4 {
            0 if at < v.len() => v[at] ^= 1 << (self.next() % 8),
            1 if at < v.len() => v[at] = self.next() as u8,
            2 => v.truncate(at),
            _ => v.insert(at.min(v.len()), self.next() as u8),
        }
    }
}

fn file(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/vectors/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

#[test]
fn damaged_keyrings_and_signatures_never_panic() {
    let data: Vec<u8> = (0..70000u32).map(|i| ((i * 131 + 17) % 251) as u8).collect();
    let mut r = Rng(0x0DD5_EED5);
    for (name, sig) in [("ed", "ed-sha512.sig"), ("sub", "sub-sha512.sig")] {
        let (pubkey, signature) = (file(&format!("{name}.pub")), file(sig));
        let ring = keys(&pubkey).unwrap_or_default();
        for _ in 0..ROUNDS {
            let mut k = pubkey.clone();
            r.damage(&mut k);
            let _ = keys(&k);
            let mut s = signature.clone();
            r.damage(&mut s);
            let _ = verify(&Host, &ring, &s, &data);
        }
    }
}
