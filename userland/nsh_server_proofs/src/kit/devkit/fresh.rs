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

//! The devkit's keys are its own: every host, authority, role-table and
//! user key is made from a space drawn from the OS generator, mixed into the
//! builders' seeds, so nobody with this source can make them. The proofs set
//! no space and keep their fixed keys.

use crate::kit::attest_build::keys::reseed as reseed_attest;
use crate::kit::roles_build::keys::reseed as reseed_roles;
use std::fs::File;
use std::io::Read;

/// Draw a space from `/dev/urandom` and make every key from it. False if the
/// generator would not answer or a key was already made.
pub fn fresh() -> bool {
    let mut space = [0u8; 32];
    let drawn = File::open("/dev/urandom").and_then(|mut f| f.read_exact(&mut space)).is_ok();
    drawn && reseed_attest(space) && reseed_roles(space)
}
