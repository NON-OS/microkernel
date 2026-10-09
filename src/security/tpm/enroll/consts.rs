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

//! The commands and values enrollment adds to the machine key's: TPM 2.0 Part 2
//! by name, and the bounds every input and response length is held to.

pub(super) const TPM_CC_ACTIVATE_CREDENTIAL: u32 = 0x0000_0147;
pub(super) const TPM_CC_NV_READ: u32 = 0x0000_014E;
pub(super) const TPM_CC_POLICY_SECRET: u32 = 0x0000_0151;
pub(super) const TPM_CC_SIGN: u32 = 0x0000_015D;
pub(super) const TPM_CC_NV_READ_PUBLIC: u32 = 0x0000_0169;
pub(super) const TPM_CC_READ_PUBLIC: u32 = 0x0000_0173;
pub(super) const TPM_CC_HASH: u32 = 0x0000_017D;

/// The endorsement hierarchy: the EK and the attestation key both live under
/// it, and its empty auth is what policy A asks for.
pub(super) const TPM_RH_ENDORSEMENT: u32 = 0x4000_000B;
pub(super) const TPM_RS_PW: u32 = 0x4000_0009;
pub(super) const TPM_ST_HASHCHECK: u16 = 0x8024;
pub(super) const TPM_ALG_ECDSA: u16 = 0x0018;

/// Policy A, `PolicySecret(TPM_RH_ENDORSEMENT)` from an empty session: the
/// EK's only authorization for use.
pub(super) const POLICY_A: [u8; 32] = [
    0x83, 0x71, 0x97, 0x67, 0x44, 0x84, 0xB3, 0xF8, 0x1A, 0x90, 0xCC, 0x8D, 0x46, 0xA5, 0xD7, 0x24,
    0xFD, 0x52, 0xD7, 0x6E, 0x06, 0x52, 0x0B, 0x64, 0xF2, 0xA1, 0xDA, 0x1B, 0x33, 0x14, 0x69, 0xAA,
];

/// `fixedTPM | fixedParent | sensitiveDataOrigin | adminWithPolicy |
/// restricted | decrypt`. No `userWithAuth`, so use needs policy A.
pub(super) const EK_ATTRIBUTES: u32 = 0x0003_00B2;

pub(super) const TPM_ALG_RSA: u16 = 0x0001;
pub(super) const TPM_ALG_AES: u16 = 0x0006;
pub(super) const TPM_ALG_ECC: u16 = 0x0023;
pub(super) const TPM_ALG_CFB: u16 = 0x0043;
pub(super) const TPM_ECC_NIST_P256: u16 = 0x0003;

/// `TPMA_NV_OWNERREAD`, `TPMA_NV_AUTHREAD`, `TPMA_NV_WRITTEN`, and the index
/// type field, zero for an ordinary index.
pub(super) const NV_OWNERREAD: u32 = 1 << 17;
pub(super) const NV_AUTHREAD: u32 = 1 << 18;
pub(super) const NV_WRITTEN: u32 = 1 << 29;
pub(super) const NV_TYPE_MASK: u32 = 0x0000_00F0;

/// A name: `TPM_ALG_SHA256`, then a SHA-256 digest.
pub const NAME_LEN: usize = 34;
/// Every message the AK signs for enrollment is this label, then the 32 bytes,
/// so the signature is nothing else: not a quote, whose signed structure opens
/// with TPM_GENERATED_VALUE, nor any later use of the key under another label.
/// The registrar verifies over the same bytes.
pub const AK_SIGN_LABEL: &[u8] = b"NONOS-ENROLL-AK-SIGN-v1";
/// The largest digest a TPM carries, SHA-512's.
pub(super) const DIGEST_MAX: usize = 64;
/// No EK certificate comes near this. One over it is refused, not truncated.
pub const CERT_MAX: usize = 4096;
/// TPMS_ID_OBJECT: an HMAC and the encrypted credential, each at most a
/// SHA-512 digest with its size field.
pub const ID_OBJECT_MAX: usize = 2 * (2 + DIGEST_MAX);
/// TPM2B_ENCRYPTED_SECRET: an RSA 4096 ciphertext is the largest a TPM takes.
pub const ENCRYPTED_SECRET_MAX: usize = 512;
/// Bytes per `TPM2_NV_Read`: inside the NV buffer of every part, and with the
/// header and session area the response stays inside `run`'s buffer.
pub(super) const NV_CHUNK: usize = 512;
