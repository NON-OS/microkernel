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

// SHA-1 needed for WPA compatibility - use SHA-256 or SHA-3 for new code.
// The digest is the TPM bank's (sha1_tpm.rs), here under its deprecation.
#[deprecated(
    since = "0.8.0",
    note = "SHA-1 is cryptographically broken. Use SHA-256 or SHA-3 instead."
)]
pub fn sha1(data: &[u8]) -> [u8; 20] {
    super::sha1_tpm::tpm_bank_sha1(data)
}
