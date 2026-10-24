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

//! What the fuzzing proofs share: one genuine sealed release, built once, and
//! a strategy that mutates its bytes (each change a byte set, then perhaps a
//! cut), so the parsers are driven both by noise and by near-genuine input.

use super::sealed_fixture::Sealed;
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};
use std::fs;
use std::sync::OnceLock;

pub struct Genuine {
    pub sealed: Sealed,
    pub record: Vec<u8>,
    pub receipt: Vec<u8>,
    pub kernel: Vec<u8>,
    pub ledger: Vec<u8>,
}

// The fixture's temp dirs live for the whole test binary; it is never dropped.
pub fn genuine() -> &'static Genuine {
    static G: OnceLock<Genuine> = OnceLock::new();
    G.get_or_init(|| {
        let sealed = Sealed::dev();
        let r = |rel: &str| fs::read(sealed.release.path().join(rel)).unwrap();
        let (record, receipt, kernel) =
            (r("nonos-release.json"), r("build-receipt.json"), r("kernel.bin"));
        let ledger = fs::read(sealed.tree.path().join("nonos-data/trust/MANIFEST.sha256")).unwrap();
        Genuine { sealed, record, receipt, kernel, ledger }
    })
}

/// The genuine bytes with up to `n` bytes set, then cut short or not.
pub fn mutated(genuine: &'static [u8], n: usize) -> impl Strategy<Value = Vec<u8>> {
    let len = genuine.len();
    let edits = prop::collection::vec((0..len, any::<u8>()), 1..=n);
    (edits, prop::option::of(0..=len)).prop_map(move |(edits, cut)| {
        let mut b = genuine.to_vec();
        for (at, v) in edits {
            b[at] = v;
        }
        b.truncate(cut.unwrap_or(len));
        b
    })
}

pub fn runner(cases: u32) -> TestRunner {
    TestRunner::new(Config { cases, failure_persistence: None, ..Config::default() })
}
