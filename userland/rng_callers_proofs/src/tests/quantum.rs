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

//! The quantum engine's generator (security/quantum/pqc/rng.rs): its pool
//! and every output are drawn only from the seeded RNG. With none it starts
//! unhealthy, its health check finds nothing, and it gives no bytes.

use crate::security::quantum::pqc::rng::QuantumRng;
use crate::stand_in::rng;

#[test]
fn with_no_randomness_it_starts_unhealthy_and_gives_nothing() {
    rng::refuse();
    let q = QuantumRng::new();
    assert!(!q.is_healthy());
    assert!(!q.health_check());
    assert_eq!(q.gen_bytes(16), None);
    assert_eq!(rng::weak(), 0);
}

#[test]
fn its_pool_is_drawn_and_passes_the_health_check() {
    rng::fail_from(None);
    let q = QuantumRng::new();
    assert_eq!(rng::given().iter().map(Vec::len).collect::<Vec<_>>(), [4096]);
    assert!(q.is_healthy());
    assert!(q.health_check());
}

#[test]
fn each_output_is_drawn_fresh_not_taken_from_the_pool() {
    rng::fail_from(None);
    let q = QuantumRng::new();
    let (a, b) = (q.gen_bytes(32).expect("bytes"), q.gen_bytes(32).expect("bytes"));
    let given = rng::given();
    assert_eq!((&given[1], &given[2]), (&a, &b));
    assert_ne!(a, b);
    assert!(!given[0].windows(32).any(|w| w == a));
    rng::refuse();
    assert_eq!(q.gen_bytes(32), None);
}
