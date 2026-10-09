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

//! Argon2id as a password KDF.

use super::derive::argon2;
use super::ends::Inputs;
use super::params::{Argon2Error, Params};

/// Argon2id with no secret and no associated data: the password KDF.
pub fn argon2id(
    password: &[u8],
    salt: &[u8],
    params: Params,
    out: &mut [u8],
    between: &mut dyn FnMut(),
) -> Result<(), Argon2Error> {
    let inputs = Inputs { password, salt, secret: &[], ad: &[], y: 2 };
    argon2(&inputs, params, out, between)
}
