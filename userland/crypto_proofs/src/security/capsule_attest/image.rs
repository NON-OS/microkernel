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

//! The image-taking entry the proofs drive. The kernel measures an image once
//! (`capsule_attest::measure`) and hands only the digest to
//! `against_pedersen::verify_digest`; the proofs start from the image, so
//! they take the same BLAKE3 digest here.

use super::against_pedersen::verify_digest;
use super::error::AttestError;

pub(super) fn verify(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
    root: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    verify_digest(trailer, blake3::hash(elf).as_bytes(), granted_caps, root)
}
