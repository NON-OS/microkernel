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

//! Beside each measurement a family keeps the kinds of verification that
//! accepted it, and the seal answers a question about a measurement only for
//! the kinds asked. The kinds stay out of the record: M, its count and the
//! list are what they would be without them.

use super::fixture::family;
use crate::foreign::seal::kind::{ALL, LOCAL, PUBLISHER};
use crate::foreign::seal::query::holds;
use crate::foreign::seal::table::LIVE;

const SUP: u32 = 0x6E70;

fn with_live(sup: u32, run: impl FnOnce(&mut crate::foreign::seal::family::Family)) {
    let mut live = LIVE.lock();
    let f = live.iter_mut().find(|f| f.sup == sup).expect("a live family");
    run(f);
}

#[test]
fn a_measurement_is_answered_only_for_the_kinds_that_verified_it() {
    let (policy, program) = ([0x51; 32], [0x52; 32]);
    LIVE.lock().push(family(SUP));
    with_live(SUP, |f| {
        f.measured(&policy, PUBLISHER);
        f.measured(&program, LOCAL);
    });
    assert!(!holds(SUP, &policy, LOCAL), "a publisher's chain is no local trailer");
    assert!(holds(SUP, &policy, PUBLISHER) && holds(SUP, &policy, ALL));
    assert!(holds(SUP, &program, LOCAL) && !holds(SUP, &program, PUBLISHER));
    with_live(SUP, |f| f.measured(&policy, LOCAL));
    assert!(holds(SUP, &policy, LOCAL) && holds(SUP, &policy, PUBLISHER), "both kinds kept");
    assert!(!holds(SUP, &policy, 0), "no kind asked, no answer");
    LIVE.lock().retain(|f| f.sup != SUP);
}

#[test]
fn the_kinds_leave_the_record_as_it_was() {
    let (a, b) = ([0x61; 32], [0x62; 32]);
    let mut plain = family(SUP + 1);
    let mut kinded = family(SUP + 1);
    plain.measured(&a, LOCAL);
    plain.measured(&b, LOCAL);
    kinded.measured(&a, PUBLISHER);
    kinded.measured(&b, LOCAL);
    kinded.measured(&a, LOCAL);
    assert_eq!((plain.meas, plain.n_meas, &plain.list), (kinded.meas, kinded.n_meas, &kinded.list));
    assert_eq!(kinded.kinds, [PUBLISHER | LOCAL, LOCAL]);
    assert_eq!(kinded.list.len(), kinded.kinds.len());
}

#[test]
fn every_kind_is_one_bit_and_all_is_their_union() {
    assert_eq!(LOCAL & PUBLISHER, 0);
    assert_eq!(ALL, LOCAL | PUBLISHER);
    assert!(LOCAL.is_power_of_two() && PUBLISHER.is_power_of_two());
}
