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

use super::ask::{ask_for, Caller, ProveAsk, DELIVER, PROVE};
use super::slots::{Next, Slots};

const MIN: u64 = 60 * 1000;
const LINUX: [u8; 32] = [0x11; 32];

pub fn caller(pid: u32, owns_store: bool, vendor: bool, live: bool) -> Caller {
    let vendor = vendor.then_some(LINUX);
    Caller { pid, owns_store, vendor, supervises_live: live, launcher: false }
}

#[test]
fn deliver_needs_a_vendor_supervising_a_live_family() {
    let ok = ask_for(DELIVER, &caller(9, false, true, true));
    assert_eq!(ok, Some(ProveAsk::Deliver(9, LINUX)));
    assert_eq!(ask_for(DELIVER, &caller(9, false, true, false)), None, "no live family");
    assert_eq!(ask_for(DELIVER, &caller(9, true, false, true)), None, "not Vendor");
    assert_eq!(ask_for(DELIVER, &caller(9, true, false, false)), None);
}

#[test]
fn prove_needs_the_store_and_other_kinds_are_none() {
    assert_eq!(ask_for(PROVE, &caller(3, true, false, false)), Some(ProveAsk::Prove(3)));
    assert_eq!(ask_for(PROVE, &caller(3, false, true, true)), None, "a Vendor is not the store");
    for kind in [0, 3, u64::MAX] {
        assert_eq!(ask_for(kind, &caller(3, true, true, true)), None, "kind {kind}");
    }
}

#[test]
fn two_supervisors_of_one_measurement_share_the_rate() {
    let mut s = Slots::new();
    assert!(s.ask(ProveAsk::Deliver(20, LINUX), MIN));
    assert!(!s.ask(ProveAsk::Deliver(21, LINUX), MIN), "one waits already");
    assert_eq!(s.next(), Some(Next::Deliver(20)));
    assert!(
        !s.ask(ProveAsk::Deliver(21, LINUX), 2 * MIN),
        "the other supervisor, same measurement"
    );
    assert!(s.ask(ProveAsk::Deliver(22, [0x22; 32]), 2 * MIN), "another measurement");
    assert_eq!(s.next(), Some(Next::Deliver(22)));
    assert!(s.ask(ProveAsk::Deliver(21, LINUX), 11 * MIN), "ten minutes on");
}

#[test]
fn prove_goes_first_and_each_kind_waits_once() {
    let mut s = Slots::new();
    assert!(!s.waiting());
    assert!(s.ask(ProveAsk::Deliver(20, LINUX), MIN));
    assert!(s.ask(ProveAsk::Prove(3), MIN));
    assert!(!s.ask(ProveAsk::Prove(4), MIN), "one PROVE waits already");
    assert!(s.waiting());
    assert_eq!(s.next(), Some(Next::Prove(3)));
    assert_eq!(s.next(), Some(Next::Deliver(20)));
    assert_eq!(s.next(), None);
    assert!(!s.waiting());
}
