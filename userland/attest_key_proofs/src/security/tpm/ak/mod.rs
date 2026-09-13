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

#[path = "../../../../../../src/security/tpm/ak/cursor.rs"]
mod cursor;
#[path = "../../../../../../src/security/tpm/ak/public.rs"]
pub mod public;
#[path = "../../../../../../src/security/tpm/ak/attributes.rs"]
mod attributes;

/// The kernel keeps the parser private to the module; this is the seam the
/// tests reach it through.
pub fn parse_public(resp: &[u8]) -> Result<[u8; 64], super::error::TpmError> {
    public::parse_public(resp)
}
