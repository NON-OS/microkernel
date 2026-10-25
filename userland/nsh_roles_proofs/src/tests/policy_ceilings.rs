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

//! A role's ceilings name only capabilities the kernel has, never one of
//! the nine no role may hold, and the login ceiling is never empty.

use crate::build::table::{RoleB, TableB};
use nsh_roles::caps::FORBIDDEN;
use nsh_roles::{open_policy, PolicyError};

fn with(f: impl FnOnce(&mut RoleB)) -> Result<(), PolicyError> {
    let mut t = TableB::default();
    f(t.role("deploy"));
    open_policy(&t.bytes(), &t.anchor, 0).map(|_| ())
}

#[test]
fn each_forbidden_name_is_refused_in_a_ceiling() {
    for name in FORBIDDEN {
        assert_eq!(with(|r| r.ceiling.push(name)), Err(PolicyError::CapForbidden), "{name}");
    }
}

#[test]
fn each_forbidden_name_is_refused_in_an_escalation_ceiling() {
    for name in FORBIDDEN {
        assert_eq!(with(|r| r.escalation.push(name)), Err(PolicyError::CapForbidden), "{name}");
    }
}

#[test]
fn a_forbidden_name_alone_is_refused() {
    assert_eq!(with(|r| r.ceiling = vec!["Admin"]), Err(PolicyError::CapForbidden));
    assert_eq!(with(|r| r.escalation = vec!["Keyring"]), Err(PolicyError::CapForbidden));
}

#[test]
fn a_name_the_kernel_lacks_is_refused() {
    assert_eq!(with(|r| r.ceiling.push("Root")), Err(PolicyError::CapUnknown));
    assert_eq!(with(|r| r.escalation.push("Root")), Err(PolicyError::CapUnknown));
}

#[test]
fn a_login_ceiling_is_never_empty_and_an_escalation_ceiling_may_be() {
    assert_eq!(with(|r| r.ceiling.clear()), Err(PolicyError::EmptyCeiling));
    assert_eq!(with(|r| r.escalation.clear()), Ok(()));
}

#[test]
fn every_other_kernel_name_may_be_in_a_ceiling() {
    let allowed: Vec<_> =
        nsh_roles::caps::KERNEL_CAPS.iter().copied().filter(|n| !FORBIDDEN.contains(n)).collect();
    assert_eq!(allowed.len(), 31);
    let r = with(|r| {
        r.ceiling = allowed.clone();
        r.escalation = allowed.clone();
    });
    assert_eq!(r, Ok(()));
}
