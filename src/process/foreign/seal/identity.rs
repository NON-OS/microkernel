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

//! Who the supervisor is, as the spawn gate recorded it: measurement, caps
//! and the authority byte the registry folds into its root.

use crate::security::attest_registry::attested_of;

/// Zeros for a supervisor no gate proved: its record then names no program,
/// and a verifier that needs one refuses it.
pub(crate) fn identity(sup: u32) -> ([u8; 32], u64, u8) {
    match attested_of(sup) {
        Some(e) => (e.measurement, e.caps, e.digest_input()[44]),
        None => ([0; 32], 0, 0),
    }
}
