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

//! zerostate's `terminate` and the panic path, together: `terminate` reads
//! the panic stage before its own once-guard, so over every order of
//! terminate and panic entries by two CPUs, at most one leave runs, at most
//! one panic path runs, and no leave starts once the panic path is taken.
//! A panic after a leave began escalates to the panic path, as documented.

use super::helpers::{enter, terminate, Did};
use super::stage::{Next, IDLE};

#[test]
fn every_order_of_terminate_and_panic_entries_by_two_cpus() {
    // Each of up to five entries: terminate or panic, by cpu 1 or 2.
    for code in 0..4u32.pow(5) {
        let (mut stage, mut ending) = (IDLE, false);
        let mut did = Vec::new();
        for k in 0..5 {
            let e = code / 4u32.pow(k) % 4;
            let cpu = (e % 2 + 1) as u64;
            did.push(if e < 2 {
                terminate(stage, &mut ending)
            } else {
                Did::Panic(enter(&mut stage, cpu))
            });
        }
        let leaves = did.iter().filter(|d| **d == Did::Leave).count();
        let fulls = did.iter().filter(|d| **d == Did::Panic(Next::Full)).count();
        assert!(leaves <= 1 && fulls <= 1, "{code}: {did:?}");
        if let Some(p) = did.iter().position(|d| matches!(d, Did::Panic(Next::Full))) {
            assert!(
                !did[p..].contains(&Did::Leave),
                "{code}: a leave after the panic path: {did:?}"
            );
        }
    }
}

#[test]
fn a_terminate_after_a_panic_parks_and_a_panic_after_a_leave_escalates() {
    let (mut stage, mut ending) = (IDLE, false);
    assert_eq!(enter(&mut stage, 1), Next::Full);
    assert_eq!(terminate(stage, &mut ending), Did::Parked);
    assert!(!ending, "a parked terminate takes no once-guard");
    let (mut stage, mut ending) = (IDLE, false);
    assert_eq!(terminate(stage, &mut ending), Did::Leave);
    assert_eq!(enter(&mut stage, 1), Next::Full);
}
