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

//! Persistent in the policy service and the kernel's keep latch give the
//! same answer after every set, as vfs now asks the kernel.

use crate::keep_once::may_set;
use crate::keep_rule::{after, may_arm, Policy};
use crate::sweep::Rng;

const POLICY: Policy = Policy { pid: 7, alive: true };

#[test]
fn persistent_turns_on_and_stays_on() {
    assert!(may_set(false, true));
    assert!(may_set(false, false));
    assert!(may_set(true, true));
    assert!(!may_set(true, false));
}

/* One set of Persistent by the service, with the push the service makes
 * when it turns on: the kernel's latch moves only through `after`. */
fn step(state: (bool, bool), value: bool) -> (bool, bool) {
    let (persistent, armed) = state;
    if !may_set(persistent, value) {
        return state;
    }
    let armed = if value { after(armed, may_arm(7, POLICY, true)) } else { armed };
    (value, armed)
}

#[test]
fn random_sets_leave_the_service_and_the_kernel_agreeing() {
    let mut r = Rng::new(0x5eed_0007);
    let mut turned_on = 0;
    for _ in 0..20_000 {
        let mut state = (false, false);
        for _ in 0..r.below(12) {
            state = step(state, r.below(2) == 1);
            assert_eq!(state.0, state.1, "service {} kernel {}", state.0, state.1);
        }
        turned_on += state.0 as u32;
    }
    assert!(turned_on > 0, "a boot that keeps was never reached");
}
