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

//! A chamber's key, signature half and generator state, and a quantum
//! chamber's key: each drawn; a refused draw makes no chamber. Findings are
//! pinned as the code has them (see each test).

use super::super::types::CapabilityEngine;
use super::engine;
use crate::security::policy::capability::isolation::IsolationLevel;
use crate::stand_in::rng;

fn chambers(e: &CapabilityEngine) -> usize {
    e.chambers.read().len()
}

/// Finding: the chamber's id is taken before its key is drawn, so a
/// refused chamber spends one.
#[test]
fn no_chamber_is_made_when_any_of_its_draws_refuses() {
    let e = engine();
    for (level, draws) in [(IsolationLevel::Basic, 3), (IsolationLevel::QuantumSecure, 4)] {
        for (from, only) in rng::cases(draws) {
            rng::fail_at(Some(from), only);
            assert_eq!(e.create_isolation_chamber(level, &[]), Err("no randomness"));
            assert_eq!(chambers(&e), 0);
        }
    }
    rng::fail_from(None);
    assert_eq!(e.create_isolation_chamber(IsolationLevel::Basic, &[]), Ok(1 + 2 * (3 + 4)));
}

#[test]
fn a_chamber_holds_its_draws() {
    let e = engine();
    let id = e.create_isolation_chamber(IsolationLevel::Basic, &[]).expect("chamber");
    let other = e.create_isolation_chamber(IsolationLevel::Basic, &[]).expect("chamber");
    let all = e.chambers.read();
    let c = &all[&id];
    let made =
        [&c.memory_encryption_key[..], &c.chamber_signature[..32], &c.secure_rng_state.lock()[..]];
    assert_eq!(rng::given()[..3], made.map(<[u8]>::to_vec));
    assert_eq!(&c.chamber_signature[32..], &e.signing_key);
    assert_ne!(c.memory_encryption_key, all[&other].memory_encryption_key);
    assert_eq!(rng::weak(), 0);
}

/// Finding: a quantum chamber's key is drawn, but its particles' numbers
/// take the weak word, 20 of them; they are no key.
#[test]
fn a_quantum_chamber_draws_its_key_and_takes_weak_particle_numbers() {
    let e = engine();
    let id = e.create_isolation_chamber(IsolationLevel::QuantumSecure, &[]).expect("chamber");
    let all = e.chambers.read();
    let state = all[&id].quantum_entanglement.as_ref().expect("quantum state");
    assert_eq!(rng::given()[1], state.quantum_key);
    assert_eq!(rng::weak(), 20);
}
