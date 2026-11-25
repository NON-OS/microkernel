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

//! The binding the personality's app::bind makes, standing in: an identity
//! named by the program's hash. Like app/bind.rs, it has the kernel verify
//! the policy file beside the program, which the kernel then records for
//! the caller's family (seal/absorb.rs), so `POLICY` stands for that file.

use super::launch_seal_tests::{kernel_measured, SUPERVISOR};

/// The bytes of the policy file beside the test's program.
pub const POLICY: &[u8] = b"NLXPOL01 a vouched policy";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppIdentity {
    pub program: [u8; 32],
    pub built_in: bool,
}

pub fn bind(_path: &[u8], image: &[u8]) -> Result<AppIdentity, &'static str> {
    kernel_measured(SUPERVISOR, blake3::hash(POLICY).as_bytes());
    Ok(AppIdentity { program: *blake3::hash(image).as_bytes(), built_in: false })
}

pub fn bind_built_in(image: &[u8]) -> AppIdentity {
    AppIdentity { program: *blake3::hash(image).as_bytes(), built_in: true }
}
