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

//! A policy's boot applications: at most 64, none zero.

use nsh_attest::policy::MAX_BOOT_APPS;
use nsh_attest::{Policy, PolicyError};

use crate::build::keys::{hybrid, AUTHORITY};
use crate::build::parts::release;

#[test]
fn boot_applications_over_the_cap_or_zero_are_refused() {
    let p = || make_policy();
    assert!(p().with_boot_applications(vec![[1; 32]; MAX_BOOT_APPS]).is_ok());
    let r = p().with_boot_applications(vec![[1; 32]; MAX_BOOT_APPS + 1]).map(|_| ());
    assert_eq!(r, Err(PolicyError::BootApplications));
    let r = p().with_boot_applications(vec![[1; 32], [0; 32]]).map(|_| ());
    assert_eq!(r, Err(PolicyError::BootApplications));
}

fn make_policy() -> Policy {
    let auth = vec![hybrid(AUTHORITY).authority()];
    Policy::new([1; 16], auth, vec![release()], vec![]).expect("policy")
}
