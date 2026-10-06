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

//! An `MkEnroll` call read into what it asks the TPM for, or refused, before
//! the TPM is asked anything.

use super::codec::{OP_ACTIVATE, OP_AK_PUBLIC, OP_AK_SIGN, OP_EK_CERTIFICATE, OP_EK_PUBLIC};
use crate::security::tpm::enroll::EkKind;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Request<'a> {
    EkPublic(EkKind),
    EkCertificate(EkKind),
    AkPublic,
    /// The EK the challenge was made for, its ID object's body, and its
    /// encrypted secret's body.
    Activate(EkKind, &'a [u8], &'a [u8]),
    AkSign(&'a [u8; 32]),
}

/// `None` for an unknown operation, an EK kind other than 0 or 1, a kind on an
/// AK operation, input on one that takes none, a message not 32 bytes long, or
/// a challenge not framed as `challenge` reads it.
pub(super) fn request(op: u64, kind: u64, input: &[u8]) -> Option<Request<'_>> {
    let bare = input.is_empty();
    Some(match op {
        OP_EK_PUBLIC if bare => Request::EkPublic(ek_kind(kind)?),
        OP_EK_CERTIFICATE if bare => Request::EkCertificate(ek_kind(kind)?),
        OP_AK_PUBLIC if bare && kind == 0 => Request::AkPublic,
        OP_ACTIVATE => {
            let (blob, secret) = challenge(input)?;
            Request::Activate(ek_kind(kind)?, blob, secret)
        }
        OP_AK_SIGN if kind == 0 => Request::AkSign(input.try_into().ok()?),
        _ => return None,
    })
}

/// 0 for the RSA 2048 EK, 1 for the ECC P-256 one.
fn ek_kind(kind: u64) -> Option<EkKind> {
    match kind {
        0 => Some(EkKind::Rsa2048),
        1 => Some(EkKind::EccP256),
        _ => None,
    }
}

/// The TPM2B_ID_OBJECT's body and the TPM2B_ENCRYPTED_SECRET's, each after
/// its length as two little-endian bytes, and nothing after them. The TPM's
/// bounds on each are `activate_credential`'s to hold.
fn challenge(input: &[u8]) -> Option<(&[u8], &[u8])> {
    let (blob, rest) = sized(input)?;
    let (secret, rest) = sized(rest)?;
    rest.is_empty().then_some((blob, secret))
}

fn sized(b: &[u8]) -> Option<(&[u8], &[u8])> {
    let n = usize::from(u16::from_le_bytes([*b.first()?, *b.get(1)?]));
    Some((b.get(2..2 + n)?, b.get(2 + n..)?))
}
