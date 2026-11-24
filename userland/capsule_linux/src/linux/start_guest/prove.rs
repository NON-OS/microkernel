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

//! A program proved and bound by its start, after its guest exists. The
//! kernel records what it verifies in the family's seal, and a family exists
//! only from its first guest (src/process/foreign/seal/open.rs): a proof
//! taken before the spawn, or a result kept from one, leaves the program out
//! of the run's record. The role is chosen earlier on a binding alone
//! (bind_role.rs), which asks the kernel to verify nothing.

use super::super::app::{bind, bind_built_in, AppIdentity};
use super::super::origin::Origin;

/// A program out of the store proves itself against the enrolled set, and
/// either kind is bound to the policy it runs under.
pub(in crate::linux) fn prove(
    path: &[u8],
    bytes: &[u8],
    origin: &Origin,
) -> Result<AppIdentity, &'static str> {
    match origin {
        Origin::BuiltIn => Ok(bind_built_in(bytes)),
        Origin::Store => super::super::attest::verify(path, bytes).and_then(|_| bind(path, bytes)),
    }
}
