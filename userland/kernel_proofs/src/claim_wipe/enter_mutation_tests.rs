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

//! Plan section 6's undo checks 2, 3, 7, 11 and 12, each a knob on the enter
//! model (L78). On the fates each is run over, the model as built breaks
//! nothing, while the knob breaks the invariants named for it in some order;
//! each sweep's worlds are counted against a floor.

use std::collections::BTreeSet;

use super::enter_check::{I1, I5, I7, I9, LEFT, PARKED, STRANDED};
use super::enter_fates::FATES;
use super::enter_parts::Knob;
use super::enter_search::sweep;

/// What `k` breaks over the fates named, after the build is shown to break
/// nothing there; at least `floor` worlds searched under the knob.
fn undone(k: Knob, fates: &[&str], floor: usize) -> BTreeSet<&'static str> {
    let (mut broken, mut worlds) = (BTreeSet::new(), 0);
    for &(name, fate) in FATES.iter().filter(|f| fates.contains(&f.0)) {
        assert!(sweep(fate, Knob::Built).broken.is_empty(), "{name} as built");
        let f = sweep(fate, k);
        (worlds, broken) = (worlds + f.worlds, broken.union(&f.broken).copied().collect());
    }
    assert!(worlds >= floor, "{k:?}: {worlds} worlds");
    broken
}

#[test]
fn admits_asked_apart_from_the_push_lets_two_services_in() {
    let broken = undone(Knob::SplitAdmit, &["clean", "late"], 450_000);
    assert!(broken.contains(I1), "{broken:?}");
}

#[test]
fn an_owe_apart_from_the_removal_says_resetting_over_nothing() {
    let broken = undone(Knob::OweApart, &["clean", "no way"], 180_000);
    assert!(broken.contains(I9), "{broken:?}");
}

#[test]
fn a_reset_waited_out_inside_making_is_stranded_by_a_kill() {
    let broken = undone(Knob::ResetInMaking, &["clean", "never"], 200_000);
    assert!([I5, PARKED, STRANDED, LEFT].iter().all(|i| broken.contains(i)), "{broken:?}");
}

#[test]
fn held_written_at_enter_launders_a_kept_function_on_withdraw() {
    let broken = undone(Knob::HeldAtEnter, &["display", "no way", "ignores"], 300_000);
    assert!(broken.contains(I7) && broken.contains(I1), "{broken:?}");
}

#[test]
fn a_claimant_waiting_in_its_entry_is_stranded_by_a_kill() {
    let broken = undone(Knob::WaitInMaking, &["clean", "no way"], 600_000);
    assert!([I5, PARKED, STRANDED].iter().all(|i| broken.contains(i)), "{broken:?}");
    // The tick still ends the wipe: only the claimant's own entry is lost.
    assert!(!broken.contains(LEFT), "{broken:?}");
}
