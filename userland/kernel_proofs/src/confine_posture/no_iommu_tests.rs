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

//! The no-IOMMU posture: a machine whose firmware left VT-d or AMD-Vi off
//! boots with its devices unconfined and marked, never one with a unit in
//! service, never a Hardened or Air-Gapped boot, never a build without it.

use super::decide::{verdict, Verdict};
use super::no_iommu_rule::{enters, reason, refusing_boot, Reason};

const B: [bool; 2] = [false, true];

fn domains() -> [Result<(), bool>; 3] {
    [Ok(()), Err(false), Err(true)]
}

#[test]
fn the_posture_is_taken_only_built_with_no_unit_and_an_open_boot() {
    for built in B {
        for unit in B {
            for refusing in B {
                assert_eq!(enters(built, unit, refusing), built && !unit && !refusing);
            }
        }
    }
    assert!(enters(true, false, false));
    assert!(!enters(true, true, false), "a unit in service always confines");
    assert!(!enters(false, false, false), "a hardened build has no posture");
    assert!(!enters(true, false, true), "a Hardened boot refuses");
}

#[test]
fn hardened_and_air_gapped_boots_refuse() {
    assert!(refusing_boot(true, false) && refusing_boot(false, true));
    assert!(!refusing_boot(false, false));
    assert_eq!(reason(false), Reason::NoTable);
    assert_eq!(reason(true), Reason::UnitDown);
    assert_ne!(Reason::NoTable.name(), Reason::UnitDown.name());
}

/// Every input the claim can see: the posture opens exactly the requester
/// whose domain failed because no unit is in service, and nothing else.
#[test]
fn the_posture_opens_only_a_bus_master_with_no_unit_in_service() {
    for rid in B {
        for covered in B {
            for domain in domains() {
                let closed = verdict(rid, covered, domain, false, (false, false), false);
                let marked = verdict(rid, covered, domain, false, (false, false), true);
                let want = if rid && domain == Err(true) { Verdict::Unconfined } else { closed };
                assert_eq!(marked, want, "rid={rid} covered={covered} {domain:?}");
                // A production build keeps the posture; it is not the opt-in.
                assert_eq!(verdict(rid, covered, domain, false, (false, true), true), want);
            }
        }
    }
    assert_eq!(verdict(true, true, Err(false), false, (false, false), true), Verdict::Refuse);
    assert_eq!(verdict(true, false, Ok(()), false, (false, false), true), Verdict::Refuse);
    assert_eq!(verdict(false, false, Ok(()), false, (false, false), true), Verdict::NoDma);
}
