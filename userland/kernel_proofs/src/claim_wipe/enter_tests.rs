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

//! The enter model as built (L78): in every order of A's give_back and exit,
//! the wipe stepped to its end by any stepper, B's and A2's claims, a fault
//! and a kill, over each fate of the device, every invariant holds; and the
//! cases that matter are reached, counted against floors.

use std::sync::OnceLock;

use super::enter_events::{REARMED, REFUSED, WITHDRAWN};
use super::enter_fates::{kept, Fate, FATES};
use super::enter_parts::Knob;
use super::enter_search::{sweep, Found};

/// Each fate's search, run once for all the tests here.
pub(super) fn built() -> &'static [(&'static str, Fate, Found)] {
    static ALL: OnceLock<Vec<(&str, Fate, Found)>> = OnceLock::new();
    ALL.get_or_init(|| FATES.iter().map(|&(n, f)| (n, f, sweep(f, Knob::Built))).collect())
}

#[test]
fn every_order_of_every_fate_keeps_every_invariant() {
    let (mut worlds, mut ends) = (0, 0);
    for (name, _, f) in built() {
        assert!(f.broken.is_empty(), "{name}: {:?}", f.broken);
        (worlds, ends) = (worlds + f.worlds, ends + f.ends);
    }
    assert!(worlds >= 900_000 && ends >= 1_500, "{worlds} worlds, {ends} ends");
}

#[test]
fn another_service_is_admitted_only_after_a_clean_reset() {
    for (name, fate, f) in built() {
        let clean = !kept(*fate) && fate.back < u64::MAX;
        assert!(if clean { f.b_in >= 100 } else { f.b_in == 0 }, "{name}: B in {}", f.b_in);
    }
}

#[test]
fn its_own_service_is_admitted_on_kept_and_nobody_on_never_back() {
    for (name, fate, f) in built() {
        let floor = if fate.back == u64::MAX { 0 } else { 30 };
        assert!(f.a2_in >= floor, "{name}: A2 in {}", f.a2_in);
        assert!(fate.back < u64::MAX || f.a2_in + f.b_in == 0, "{name}: {f:?}");
    }
}

#[test]
fn a_lost_function_is_refused_for_its_hold_then_asked_again() {
    let lost = built().iter().filter(|b| matches!(b.0, "late" | "never"));
    lost.for_each(|(n, _, f)| assert_eq!(f.events & (REFUSED | REARMED), REFUSED | REARMED, "{n}"));
}

#[test]
fn withdraws_are_reached_and_held_to_i7() {
    for (name, fate, f) in built() {
        let goes = fate.back < u64::MAX;
        assert!(!goes || f.events & WITHDRAWN != 0, "{name}: no claim withdrawn");
    }
}
