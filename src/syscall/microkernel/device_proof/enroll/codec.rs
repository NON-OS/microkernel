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

//! `MkEnroll`'s operations and the bytes it answers with, with no kernel in
//! them, so a host test holds the framing libc writes and reads.

use alloc::vec::Vec;

use crate::security::tpm::enroll::{Public, ENCRYPTED_SECRET_MAX, ID_OBJECT_MAX, NAME_LEN};

/// The EK's public area; `kind` names the EK, no input.
pub(super) const OP_EK_PUBLIC: u64 = 1;
/// The EK's certificate, DER; `kind` names the EK, no input.
pub(super) const OP_EK_CERTIFICATE: u64 = 2;
/// The AK's public area; `kind` zero, no input.
pub(super) const OP_AK_PUBLIC: u64 = 3;
/// The secret in a registrar's challenge; `kind` names the EK it was made for.
pub(super) const OP_ACTIVATE: u64 = 4;
/// The AK's signature over a 32-byte message; `kind` zero.
pub(super) const OP_AK_SIGN: u64 = 5;

/// The longest input any operation takes: a challenge at both of its bounds.
pub(super) const INPUT_MAX: usize = 2 + ID_OBJECT_MAX + 2 + ENCRYPTED_SECRET_MAX;

/// A public area as the caller gets it: the 34-byte name, then the
/// TPM2B_PUBLIC whole, size field first.
pub(super) fn public_bytes(p: &Public) -> Vec<u8> {
    let mut out = Vec::with_capacity(NAME_LEN + p.area.len());
    out.extend_from_slice(&p.name);
    out.extend_from_slice(&p.area);
    out
}
