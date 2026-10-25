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

//! The seeded model of plan section 6 (L78): 5,000 seeds for each reset
//! method and for a function offering none, every invariant judged on every
//! move of every seed, and floors on the cases reached (model_floors), so
//! that no sweep passes for want of them.

use super::method_rule::Method;
use super::model_floors::{holds, RESETS};
use super::world_cases::{B_CLEAN, CLEAN, FALLBACK, LOST, UNRESET};

#[test]
fn flr_wipes_hold_every_invariant() {
    assert!(holds(Some(Method::Flr), &RESETS)[FALLBACK] >= 400, "no power cycle after an FLR");
}

#[test]
fn advanced_features_flr_wipes_hold_every_invariant() {
    assert!(holds(Some(Method::AfFlr), &RESETS)[FALLBACK] >= 400, "no power cycle after it");
}

#[test]
fn power_cycle_wipes_hold_every_invariant() {
    assert_eq!(holds(Some(Method::PmCycle), &RESETS)[FALLBACK], 0, "nothing after a cycle");
}

#[test]
fn a_function_with_no_reset_goes_only_to_its_own_service() {
    let s = holds(None, &[(UNRESET, 6_000)]);
    assert_eq!((s[B_CLEAN], s[CLEAN], s[LOST]), (0, 0, 0), "nothing reset: {s:?}");
}
