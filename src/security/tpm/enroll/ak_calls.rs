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

//! The attestation key's part in enrollment: its public area, and its
//! signature over the message the registrar asks it to sign.

use super::ak_public::{build_read_public, parse_read_public};
use super::error::EnrollError;
use super::hash::{build_hash, parse_hash};
use super::public::Public;
use super::sign::{build_sign, parse_sign};
use crate::security::tpm::ak::load_ak;
use crate::security::tpm::machine_key::consts::DIGEST_LEN;
use crate::security::tpm::machine_key::run::run;

/// The AK's public area and name: the name goes into the registrar's
/// MakeCredential, the area is what it checks [`ak_sign`] with.
pub fn ak_public() -> Result<Public, EnrollError> {
    let ak = load_ak()?;
    parse_read_public(&run(&build_read_public(ak))?)
}

/// ECDSA P-256 over SHA-256 of `AK_SIGN_LABEL || msg`, r then s. The AK is
/// restricted, so the TPM signs only a digest it hashed itself: the hash goes
/// through the TPM first, over the label and the message, so no message can
/// make the signature pass for an attestation.
pub fn ak_sign(msg: &[u8; DIGEST_LEN]) -> Result<[u8; 64], EnrollError> {
    let ak = load_ak()?;
    let (digest, ticket) = parse_hash(&run(&build_hash(msg))?)?;
    parse_sign(&run(&build_sign(ak, &digest, &ticket))?)
}
