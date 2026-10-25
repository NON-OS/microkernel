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

//! the reset's order and bounds, and no FLR where none is offered.
//! Bus Master is clear before reset(); frames are freed after it returns.

use super::fake::{Fake, Step};
use super::reset_rule::{af_flr, pcie_flr, Flr, Reset, PENDING_MS, POLL_MS, READY_MS, SETTLE_MS};
use super::reset_run::reset;

const NEVER: u64 = u64::MAX / 2;

#[test]
fn pending_save_flr_settle_ready_restore_in_that_order() {
    let mut f = Fake::new(Some(Flr::Pcie), 25, 30);
    assert_eq!(reset(&mut f), Reset::Done { pending_left: false });
    let at = |s: Step| f.steps.iter().position(|x| *x == s).unwrap();
    let flr = at(Step::Initiate(Flr::Pcie));
    assert!(f.steps[..flr]
        .iter()
        .all(|s| matches!(s, Step::Pending(_) | Step::Pause(_) | Step::Save)));
    assert_eq!(f.steps[flr - 1], Step::Save, "the header saved last before the FLR");
    assert_eq!(f.steps[flr + 1], Step::Pause(SETTLE_MS), "nothing asked in the first 100 ms");
    assert_eq!(*f.steps.last().unwrap(), Step::Restore);
    assert!(f.steps.iter().filter(|s| matches!(s, Step::Pending(t) if *t >= 25)).count() == 1);
}

#[test]
fn transactions_that_never_finish_are_waited_out_then_reset() {
    let mut f = Fake::new(Some(Flr::Af), NEVER, 0);
    assert_eq!(reset(&mut f), Reset::Done { pending_left: true });
    let last = f.steps.iter().rev().find_map(|s| match s {
        Step::Pending(t) => Some(*t),
        _ => None,
    });
    assert!((PENDING_MS..PENDING_MS + POLL_MS).contains(&last.unwrap()), "{last:?}");
    assert!(f.steps.contains(&Step::Initiate(Flr::Af)));
}

#[test]
fn a_function_that_never_answers_is_not_restored() {
    let mut f = Fake::new(Some(Flr::Pcie), 0, NEVER);
    assert_eq!(reset(&mut f), Reset::NotReady);
    assert!(!f.steps.contains(&Step::Restore));
    let flr_at = f.reset_at.unwrap();
    assert!(
        (flr_at + SETTLE_MS + READY_MS..flr_at + SETTLE_MS + READY_MS + POLL_MS).contains(&f.now)
    );
}

#[test]
fn no_flr_offered_means_nothing_is_touched() {
    let mut f = Fake::new(None, NEVER, NEVER);
    assert_eq!(reset(&mut f), Reset::NotOffered);
    assert!(f.steps.is_empty(), "{:?}", f.steps);
}

#[test]
fn the_capability_bits() {
    assert!(pcie_flr(1 << 28) && !pcie_flr(!(1 << 28)));
    assert!(af_flr(0b11) && !af_flr(0b10) && !af_flr(0b01), "TP and FLR both");
}
