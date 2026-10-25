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

//! The three roles shipped: in order, with ceilings that name only kernel
//! capabilities in ascending order, none forbidden and none of the others a
//! login must not hold; and a table made of them opens as written.

use nsh_roles::caps::{bit, FORBIDDEN};
use nsh_roles::shipped::SHIPPED;

/// Never in a shipped role. `Crypto` and `RegisterService` may come back
/// once session capsules have their own key and service namespaces (K2).
#[rustfmt::skip]
pub const ALSO_NEVER: [&str; 15] = [
    "TimeSet", "InputConsumer", "InputSource", "SpawnBroker", "ForeignExec", "Driver", "Mmio",
    "Irq", "Dma", "Pio", "Hardware", "IO", "Entropy", "Crypto", "RegisterService",
];

#[test]
fn the_roles_are_auditor_deploy_and_operator() {
    let names: Vec<_> = SHIPPED.iter().map(|r| r.name).collect();
    assert_eq!(names, ["auditor", "deploy", "operator"]);
    assert!(names.iter().all(|n| nsh_wire::names::role(n.as_bytes()).is_ok()));
}

#[test]
fn every_ceiling_is_ascending_known_and_safe() {
    for r in &SHIPPED {
        for list in [r.ceiling, r.escalation] {
            assert!(list.windows(2).all(|w| w[0].as_bytes() < w[1].as_bytes()), "{}", r.name);
            assert!(list.iter().all(|n| bit(n.as_bytes()).is_some()), "{}", r.name);
            assert!(list.iter().all(|n| !FORBIDDEN.contains(n)), "{}", r.name);
            assert!(list.iter().all(|n| !ALSO_NEVER.contains(n)), "{}", r.name);
        }
        assert!(r.ceiling.iter().all(|n| r.escalation.contains(n) || r.escalation.is_empty()));
        for outbound in ["Network", "AppInstall"] {
            assert!(!r.ceiling.contains(&outbound), "{}: {outbound} only by escalation", r.name);
        }
    }
}

#[test]
fn the_auditor_never_escalates_and_the_others_need_an_approval() {
    let [auditor, deploy, operator] = &SHIPPED;
    assert!(auditor.escalation.is_empty());
    assert!(deploy.approvals >= 1 && operator.approvals >= 1);
}
