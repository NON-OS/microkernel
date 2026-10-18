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

//! Devices after what they depend on: an order that must be turned round,
//! a chain, a cycle (and a device waiting on it), a dangling reference,
//! a dependency outside the set that is absent, and one that is present.
//! Every device comes out exactly once.

use crate::deps::{order, Dep, Placed, Verdict};
use crate::namespace::Id;

fn ids(p: &[Placed]) -> Vec<(Id, Verdict)> {
    p.iter().map(|p| (p.id, p.verdict)).collect()
}

const OUTSIDE_PRESENT: Id = 900;
const OUTSIDE_ABSENT: Id = 901;

fn present(id: Id) -> bool {
    id == OUTSIDE_PRESENT
}

#[test]
fn after_what_they_depend_on() {
    use Verdict::Ready;
    let camera = (10, vec![Dep::On(20)]);
    let pmic = (20, vec![]);
    assert_eq!(ids(&order(&[camera.clone(), pmic.clone()], &present)), [(20, Ready), (10, Ready)]);
    let chain = [(1, vec![Dep::On(2)]), (2, vec![Dep::On(3)]), (3, vec![Dep::On(OUTSIDE_PRESENT)])];
    assert_eq!(ids(&order(&chain, &present)), [(3, Ready), (2, Ready), (1, Ready)]);
    assert_eq!(ids(&order(&[pmic], &present)), [(20, Ready)]);
    assert_eq!(order(&[], &present), []);
}

#[test]
fn unmet_is_flagged_and_passed_on() {
    use Verdict::{Ready, Unmet};
    let d = [
        (1, vec![Dep::Dangling]),
        (2, vec![Dep::On(1)]),
        (3, vec![Dep::On(OUTSIDE_ABSENT)]),
        (4, vec![Dep::On(OUTSIDE_PRESENT)]),
    ];
    assert_eq!(ids(&order(&d, &present)), [(1, Unmet), (2, Unmet), (3, Unmet), (4, Ready)]);
}

#[test]
fn cycles_come_last_once_each() {
    use Verdict::{Cycle, Ready};
    let d = [
        (1, vec![Dep::On(2)]),
        (2, vec![Dep::On(1)]),
        (3, vec![Dep::On(1)]),
        (4, vec![]),
        (5, vec![Dep::On(5)]),
    ];
    let got = ids(&order(&d, &present));
    assert_eq!(got, [(4, Ready), (1, Cycle), (2, Cycle), (3, Cycle), (5, Cycle)]);
}
