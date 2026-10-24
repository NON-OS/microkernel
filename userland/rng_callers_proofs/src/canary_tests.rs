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

// The advanced policy's stack canary (security/policy/advanced/canary.rs,
// and the manager that starts it), proved from inside its module: a value
// comes from the CPU's random instruction, else the seeded RNG, never the
// cycle counter; with neither, no canary is made, and a rotation draws
// every new value before it changes anything.

use super::super::{init_advanced_security, AdvancedSecurityManager, SecurityConfig};
use super::StackCanary;
use crate::stand_in::{arch, rng};
use core::sync::atomic::Ordering;

const NONE: &str = "no randomness for a canary";

fn word(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes.try_into().expect("8 bytes"))
}

/// The global value, the per-CPU values and the rotation count.
fn seen(c: &StackCanary) -> (u64, Vec<(u32, u64)>, u64) {
    let per_cpu = c.per_cpu.read().iter().map(|(k, v)| (*k, *v)).collect();
    (c.global.load(Ordering::SeqCst), per_cpu, c.rotations.load(Ordering::SeqCst))
}

#[test]
fn a_canary_comes_from_the_cpu_else_the_seeded_rng() {
    arch::cpu_answers(Some(77));
    rng::refuse();
    assert_eq!(StackCanary::new().map(|c| seen(&c).0), Ok(77));
    assert_eq!(rng::draws(), 0);
    arch::cpu_answers(None);
    rng::fail_from(None);
    let c = StackCanary::new().expect("canary");
    assert_eq!(seen(&c).0, word(&rng::given()[0]));
    assert_eq!(rng::weak(), 0);
}

#[test]
fn a_rotation_refused_at_any_draw_changes_nothing() {
    arch::cpu_answers(None);
    rng::fail_from(None);
    let c = StackCanary::new().expect("canary");
    c.per_cpu.write().extend([(0, 5), (1, 6)]);
    let before = seen(&c);
    for (from, only) in rng::cases(3) {
        rng::fail_at(Some(from), only);
        assert_eq!(c.rotate(), Err(NONE));
        assert_eq!(seen(&c), before);
    }
    rng::fail_from(None);
    assert_eq!(c.rotate(), Ok(()));
    let g = rng::given();
    let new = word(&g[0]);
    assert_eq!(seen(&c), (new, vec![(0, word(&g[1]) ^ new), (1, word(&g[2]) ^ new)], 1));
    assert_eq!(rng::weak(), 0);
}

include!("canary_manager_tests.rs");
