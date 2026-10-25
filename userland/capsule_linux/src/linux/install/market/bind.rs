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
//! Binding a market release's policy to the program its install recorded:
//! the policy names that program's bytes, is written beside it over the
//! PENDING that held it (minted.rs), and is vouched for by this machine.

use nonos_linux_policy::LinuxAppPolicy;

use super::super::record::vouch::sign_beside;
use super::super::recorded;
use super::check::names_program;
use super::minted::{beside, POLICY, POLICY_TRAILER};
use crate::linux::file::{key, store_read, store_write};

const MAX_IMAGE: u32 = 64 << 20;

pub(super) fn bind(pkg: &str, policy: &LinuxAppPolicy) -> Result<(), &'static str> {
    let path = recorded(pkg).ok_or("the package records no program")?;
    let image = store_read(&key(&path), MAX_IMAGE).map_err(|_| "its program could not be read")?;
    if !names_program(policy, &image) {
        return Err("its policy names another program");
    }
    store_write(&key(&beside(&path, POLICY)), &policy.encode())
        .map_err(|_| "its policy could not be written")?;
    match sign_beside(&path, &policy.encode(), POLICY_TRAILER) {
        true => Ok(()),
        false => Err("its policy could not be vouched for: no enrolled root, or may not mint"),
    }
}
