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

//! Who may turn keeping on: the policy capsule the kernel spawned, by its
//! own pid, while it runs, and only to turn it on.

use crate::keep_rule::{may_arm, Policy};
use crate::sweep::Rng;

const POLICY: Policy = Policy { pid: 7, alive: true };

#[test]
fn the_policy_capsule_turns_keeping_on() {
    assert!(may_arm(7, POLICY, true));
}

#[test]
fn any_other_capsule_is_refused() {
    for caller in [0, 1, 6, 8, 70, u32::MAX] {
        assert!(!may_arm(caller, POLICY, true), "pid {caller}");
    }
}

#[test]
fn a_dead_or_never_spawned_policy_arms_nothing() {
    assert!(!may_arm(7, Policy { pid: 7, alive: false }, true));
    // Pid 0 is the kernel's own client: no capsule is ever handed it.
    assert!(!may_arm(0, Policy { pid: 0, alive: true }, true));
    assert!(!may_arm(0, Policy { pid: 0, alive: false }, true));
}

#[test]
fn an_ask_to_turn_it_off_is_never_granted() {
    assert!(!may_arm(7, POLICY, false));
}

/* Any caller, policy pid, liveness and ask: granted exactly when it is the
 * live, spawned policy capsule asking to turn keeping on. */
#[test]
fn random_asks_are_granted_only_to_the_live_policy_turning_it_on() {
    let mut r = Rng::new(0x5eed_0004);
    let mut granted = 0;
    for _ in 0..200_000 {
        let policy = Policy { pid: r.below(8) as u32, alive: r.below(2) == 1 };
        let caller = r.below(8) as u32;
        let on = r.below(2) == 1;
        let want = on && policy.alive && policy.pid != 0 && caller == policy.pid;
        assert_eq!(may_arm(caller, policy, on), want, "{caller} {policy:?} {on}");
        granted += want as u32;
    }
    assert!(granted > 5_000, "the sweep granted too few: {granted}");
}
