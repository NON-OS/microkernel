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

//! The power-on self test: the primitives this kernel signs and seals with,
//! run against their published answers on the machine that is about to use
//! them.
//!
//! Not redundant with `userland/crypto_proofs`, which asserts the same vectors
//! in CI. A proof tests the source on a build machine. This tests the bytes
//! that are running here, so a corrupted image, a bad link or a miscompile
//! shows up before anything trusts a signature. That distinction is the only
//! reason for it to exist, and it is why it runs at boot rather than never.

use core::sync::atomic::Ordering;

use super::{kat_blake3, kat_chacha20poly1305, kat_ed25519, kat_sha3_256, AlgorithmStatus};
use super::CRYPTO_STATE;

/// The checks that hold a published answer. `kat_rng`, `kat_sphincs` and
/// `kat_ntru` are deliberately not here: they are round-trips, and a
/// round-trip reports nothing a self test can act on.
const CHECKS: [(&[u8], fn() -> AlgorithmStatus); 4] = [
    (b"sha3-256", kat_sha3_256),
    (b"blake3", kat_blake3),
    (b"chacha20-poly1305", kat_chacha20poly1305),
    (b"ed25519", kat_ed25519),
];

/// Run every check, name the ones that fail, and answer whether all passed.
pub fn run_selftest() -> bool {
    let mut failed = 0;
    for (name, check) in CHECKS.iter() {
        if check() != AlgorithmStatus::Pass {
            failed += 1;
            crate::sys::serial::print(b"[CRYPTO-POST] FAIL ");
            crate::sys::serial::println(name);
        }
    }
    CRYPTO_STATE.overall_checks_run.store(true, Ordering::SeqCst);
    crate::sys::serial::print(b"[CRYPTO-POST] ");
    crate::sys::serial::print_dec((CHECKS.len() - failed) as u64);
    crate::sys::serial::print(b"/");
    crate::sys::serial::print_dec(CHECKS.len() as u64);
    if failed == 0 {
        crate::sys::serial::println(b" known-answer tests passed");
    } else {
        crate::sys::serial::println(b" known-answer tests passed, WARNING crypto is not itself");
    }
    failed == 0
}
