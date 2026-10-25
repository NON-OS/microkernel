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

//! A release host admitted every capsule on its STARK proof under the
//! vendor's root. One its path alone admitted, one a root a person enrolled
//! admitted, and a local build are each refused by name, wherever they are.

use nsh_attest::{CapsuleFault, Refusal};

use super::super::super::refused;
use super::log_of;
use crate::build::tcg_capsule::{DEVELOPER, LOCAL_BUILD, PATH_ONLY, PROOF_VERIFIED};
use crate::build::Spec;

#[test]
fn a_capsule_not_proved_under_the_vendor_s_root_is_refused_by_name() {
    let routes = [
        (PATH_ONLY, CapsuleFault::PathOnly),
        (DEVELOPER, CapsuleFault::Developer),
        (LOCAL_BUILD, CapsuleFault::LocalBuild),
    ];
    for (kind, fault) in routes {
        for at in 0..3 {
            let mut caps = vec![(1, PROOF_VERIFIED), (2, PROOF_VERIFIED)];
            caps.insert(at, (3, kind));
            let s = Spec { log: log_of(&caps), ..Spec::default() };
            assert_eq!(refused(s), Refusal::Pcr14(fault), "kind {kind} at {at}");
        }
    }
}

#[test]
fn the_first_capsule_not_the_vendor_s_is_the_one_named() {
    let caps = [(1, PROOF_VERIFIED), (2, LOCAL_BUILD), (3, PATH_ONLY)];
    let s = Spec { log: log_of(&caps), ..Spec::default() };
    assert_eq!(refused(s), Refusal::Pcr14(CapsuleFault::LocalBuild));
}
