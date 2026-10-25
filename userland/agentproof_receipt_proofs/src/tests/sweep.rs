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
//! A whole receipt verifies, and says what it carries.
//! The seeded sweep the fuzz target shares: receipts cut, spliced, flipped
//! and filled with any byte. None panics the reader or the verifier, and
//! none verifies but the whole receipt itself.

use agentproof_receipt::napr::read::parse;
use agentproof_receipt::verify;

use crate::fixture::Fixture;

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

#[test]
fn no_mutated_receipt_panics_or_verifies() {
    let mut f = Fixture::new(b"allow:/tmp/out,net:api.example.com:443");
    let (whole, trust) = (f.bytes(), f.trust());
    let mut r = Rng(0x2ec3_5eed);
    for _ in 0..20_000 {
        let mut b = whole.clone();
        for _ in 0..1 + r.below(4) {
            match r.below(4) {
                0 => b.truncate(r.below(b.len() + 1)),
                1 => b.insert(r.below(b.len() + 1), r.next() as u8),
                2 => b.extend((0..r.below(9)).map(|_| r.next() as u8)),
                _ if !b.is_empty() => {
                    let at = r.below(b.len());
                    b[at] ^= 1 << r.below(8);
                }
                _ => {}
            }
        }
        let _ = parse(&b);
        assert!(b == whole || verify(&b, &trust).is_err(), "a mutant verified");
    }
}
