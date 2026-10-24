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

//! The quote: `TPMS_ATTEST` as the TPM signed it, and the AK's ECDSA
//! signature over it. `check_quote` takes the PCR list as an argument, so a
//! version 3 quote (PCRs 0, 1, 2 and 7) proves the signature and digest
//! code too; the document-level check passes `QUOTED_PCRS_V4` only.

mod attest;
mod check;
mod select;
mod sig;
mod types;

pub use check::check_quote;
pub use select::{bitmap, pcr_digest};
pub use sig::{EcdsaSig, TPM_ALG_ECDSA, TPM_ALG_SHA256};
pub use types::{Attest, Selection};

/// `TPM_GENERATED_VALUE`: only the TPM writes it, and a restricted key
/// signs nothing that starts with it unless the TPM built it.
pub const TPM_GENERATED: u32 = 0xFF54_4347;
/// `TPM_ST_ATTEST_QUOTE`.
pub const ST_ATTEST_QUOTE: u16 = 0x8018;
/// Bytes of PCR selection bitmap for a bank of 24 PCRs.
pub const SELECT_BYTES: usize = 3;
/// The selections a TPM may list: one per hash algorithm it implements.
pub const MAX_SELECTIONS: u32 = 16;
/// `PCR_SELECT_MAX`: room for 32 PCRs.
pub const MAX_SELECT: usize = 4;
/// The longest name or digest: a SHA-512 one, after its algorithm.
pub const MAX_HASHED: usize = 2 + 64;
