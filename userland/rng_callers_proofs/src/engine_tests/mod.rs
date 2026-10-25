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

//! The capability engine (security/policy/capability/engine/{constructor,
//! chamber,memory}.rs), proved from inside its module, where its fields are
//! in reach: every key, signature half, nonce and proof it makes is drawn
//! from the seeded RNG, and a refused draw adds nothing.

#![warn(clippy::all)]

mod chamber;
mod memory;

use super::types::CapabilityEngine;
use crate::stand_in::rng;

const NONE: &str = "no randomness for the capability engine";

/// An engine made with the RNG answering, and the draws cleared.
pub(super) fn engine() -> CapabilityEngine {
    rng::fail_from(None);
    let e = CapabilityEngine::new().expect("engine");
    rng::fail_from(None);
    e
}

#[test]
fn no_engine_is_made_when_any_of_its_draws_refuses() {
    for (from, only) in rng::cases(3) {
        rng::fail_at(Some(from), only);
        assert_eq!(CapabilityEngine::new().err(), Some(NONE));
        assert_eq!(rng::draws(), from);
    }
    assert_eq!(rng::weak(), 0);
}

#[test]
fn the_engine_keys_are_its_three_draws() {
    rng::fail_from(None);
    let e = CapabilityEngine::new().expect("engine");
    let made = [e.signing_key.to_vec(), e.quantum_rng.lock().to_vec(), e.attestation_root.to_vec()];
    assert_eq!(rng::given(), made);
    let other = CapabilityEngine::new().expect("engine");
    assert_ne!(e.signing_key, other.signing_key);
    assert_eq!(rng::weak(), 0);
}
