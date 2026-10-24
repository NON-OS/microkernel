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

//! The crypto self-test's RNG check (crypto/application/certification/
//! kat.rs): it draws two blocks from the seeded RNG, and fails when the RNG
//! cannot answer, rather than passing over weak bytes.

use crate::crypto::application::certification::{kat_rng, AlgorithmStatus, CRYPTO_STATE};
use crate::stand_in::rng;
use core::sync::atomic::Ordering;

/// The one test that touches the self-test state.
#[test]
fn the_rng_check_fails_unseeded_and_passes_seeded() {
    for (from, only) in rng::cases(2) {
        rng::fail_at(Some(from), only);
        assert_eq!(kat_rng(), AlgorithmStatus::Fail);
        assert!(!CRYPTO_STATE.rng.load(Ordering::SeqCst));
    }
    assert_eq!(CRYPTO_STATE.checks_failed.load(Ordering::SeqCst), 4);
    rng::fail_from(None);
    assert_eq!(kat_rng(), AlgorithmStatus::Pass);
    assert!(CRYPTO_STATE.rng.load(Ordering::SeqCst));
    assert_eq!(rng::given().len(), 2);
    assert_eq!(rng::weak(), 0);
}
