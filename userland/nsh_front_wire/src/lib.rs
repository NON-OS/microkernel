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

//! The IPC wire between `nsh.front` and `nsh.attest`. The split is the
//! kernel's: a quote signs the machine permanently, so the capsule that
//! asks the kernel for one (`sys_attest_doc`) must not reach the network.
//! `nsh.front` holds Network and relays each handshake's 32-byte challenge
//! here; `nsh.attest` holds AttestRead, asks the kernel, and answers with
//! the document bound to that challenge. nsh.attest also holds the host's
//! hybrid keys, made on the machine at its first boot and sealed to its
//! TPM, and signs for nsh.front, which never sees a host secret; and it
//! reads the machine's own records (enrolment, role table, revocation
//! list) for each connection. Every message is a fixed 12-byte header and
//! an optional body; nothing here allocates.

#![no_std]
#![forbid(unsafe_code)]

/// `b"NFW1"`, little-endian: the wire this header belongs to.
pub const MAGIC: u32 = 0x3157_464E;
/// The only version a peer here speaks.
pub const VERSION: u16 = 1;
/// Header bytes before the body.
pub const HDR_LEN: usize = 12;

/// Liveness only; no body, an empty OK answer.
pub const OP_HEALTH: u16 = 0;
/// Body is the 32-byte challenge; answer is the NONOSATT v5 document.
pub const OP_DOC: u16 = 1;
/// No body; answer is the attestation registry's entries.
pub const OP_ENTRIES: u16 = 2;

/// No body; answer is the host's public keys, `KEYS_LEN` bytes: the
/// Ed25519 key, then the ML-DSA-65 key.
pub const OP_KEYS: u16 = 3;
/// Body is a `DIGEST_LEN`-byte digest; answer is the host's hybrid
/// signature over it, `SIG_LEN` bytes: Ed25519, then ML-DSA-65.
pub const OP_SIGN: u16 = 4;
/// No body; answer is the machine's records (records.rs).
pub const OP_RECORDS: u16 = 5;

/// The challenge `OP_DOC` carries, and the document binds.
pub const CHALLENGE_LEN: usize = 32;

/// Reply status. A request carries `ST_OK` in the field, unread.
pub const ST_OK: u16 = 0;
/// The request header or body was not what the op requires.
pub const ST_BAD_REQUEST: u16 = 1;
/// The caller is not the one measured `nsh.front`; nothing is answered.
pub const ST_REFUSED_CALLER: u16 = 2;
/// The kernel refused to attest (no TPM, or an incomplete registry).
pub const ST_ATTEST_REFUSED: u16 = 3;
/// The answer did not fit the reply buffer the caller gave.
pub const ST_OVERFLOW: u16 = 4;
/// The machine has no TPM: there are no host keys and no attestation, and
/// nothing is faked in their place.
pub const ST_NO_TPM: u16 = 5;
/// The machine has a TPM, but the host keys could not be made or opened
/// now (the TPM refused this boot state, or the store did not answer).
pub const ST_NO_KEYS: u16 = 6;

/// The Ed25519 public key's length.
pub const ED_PUBLIC_LEN: usize = 32;
/// The ML-DSA-65 public key's length.
pub const ML_PUBLIC_LEN: usize = 1952;
/// `OP_KEYS`'s answer: both public keys.
pub const KEYS_LEN: usize = ED_PUBLIC_LEN + ML_PUBLIC_LEN;
/// The digest `OP_SIGN` carries.
pub const DIGEST_LEN: usize = 64;
/// The Ed25519 signature's length.
pub const ED_SIG_LEN: usize = 64;
/// The ML-DSA-65 signature's length.
pub const ML_SIG_LEN: usize = 3309;
/// `OP_SIGN`'s answer: both signatures.
pub const SIG_LEN: usize = ED_SIG_LEN + ML_SIG_LEN;
/// The longest request any op takes: a header and a digest.
pub const REQ_MAX: usize = HDR_LEN + DIGEST_LEN;

mod codec;
mod records;
pub use codec::{parse, write_header, Header};
pub use records::{
    records_len, split_records, write_records, Records, ANCHOR_LEN, ENROL_MAX, LIST_MAX, TABLE_MAX,
};
