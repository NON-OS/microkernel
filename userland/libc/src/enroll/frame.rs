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

//! `MkEnroll`'s framing, as `src/syscall/microkernel/enroll` reads and writes
//! it; a host test holds the two against each other.

pub const OP_EK_PUBLIC: u64 = 1;
pub const OP_EK_CERTIFICATE: u64 = 2;
pub const OP_AK_PUBLIC: u64 = 3;
pub const OP_ACTIVATE: u64 = 4;
pub const OP_AK_SIGN: u64 = 5;

/// The EK kinds: template L-1 and template L-2.
pub const EK_RSA2048: u64 = 0;
pub const EK_ECC_P256: u64 = 1;

/// `TPM_ALG_SHA256` then the digest: what MakeCredential binds.
pub const NAME_LEN: usize = 34;
/// What the AK signs is this label, then the 32 bytes; the registrar verifies over both.
pub const AK_SIGN_LABEL: &[u8] = b"NONOS-ENROLL-AK-SIGN-v1";
/// Room for the name and either EK's public area, or the AK's.
pub const PUBLIC_MAX: usize = 512;
pub const CERT_MAX: usize = 4096;
/// The activated secret, at most a SHA-512 digest.
pub const SECRET_MAX: usize = 64;
const ID_OBJECT_MAX: usize = 132;
const ENCRYPTED_SECRET_MAX: usize = 512;
pub const CHALLENGE_MAX: usize = 2 + ID_OBJECT_MAX + 2 + ENCRYPTED_SECRET_MAX;

/// The registrar's TPM2B_ID_OBJECT and TPM2B_ENCRYPTED_SECRET bodies, each
/// after its length as two little-endian bytes. The length written, or `None`
/// when either is empty or longer than its TPM structure.
pub fn frame_challenge(blob: &[u8], secret: &[u8], out: &mut [u8; CHALLENGE_MAX]) -> Option<usize> {
    if blob.is_empty() || blob.len() > ID_OBJECT_MAX {
        return None;
    }
    if secret.is_empty() || secret.len() > ENCRYPTED_SECRET_MAX {
        return None;
    }
    let mut at = 0;
    for part in [blob, secret] {
        out.get_mut(at..at + 2)?.copy_from_slice(&(part.len() as u16).to_le_bytes());
        out.get_mut(at + 2..at + 2 + part.len())?.copy_from_slice(part);
        at += 2 + part.len();
    }
    Some(at)
}

/// A public answer as the name and the TPM2B_PUBLIC, whose big-endian size
/// field must cover the rest exactly.
pub fn split_public(answer: &[u8]) -> Option<(&[u8; NAME_LEN], &[u8])> {
    let name = answer.get(..NAME_LEN)?.try_into().ok()?;
    let area = answer.get(NAME_LEN..)?;
    let n = usize::from(u16::from_be_bytes([*area.first()?, *area.get(1)?]));
    (area.len() == 2 + n).then_some((name, area))
}
