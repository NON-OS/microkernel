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

//! The commands and constants beyond the machine key's, from TPM 2.0 Part 2.

pub(super) const TPM_CC_LOAD_EXTERNAL: u32 = 0x0000_0167;
pub(super) const TPM_CC_VERIFY_SIGNATURE: u32 = 0x0000_0177;
pub(super) const TPM_CC_POLICY_AUTHORIZE: u32 = 0x0000_016A;

pub(super) const TPM_ALG_ECC: u16 = 0x0023;
pub(super) const TPM_ALG_ECDSA: u16 = 0x0018;
pub(super) const TPM_ALG_SHA256: u16 = 0x000B;
pub(super) const TPM_ALG_NULL: u16 = 0x0010;
pub(super) const TPM_ECC_NIST_P256: u16 = 0x0003;

pub(super) const TPM_ST_VERIFIED: u16 = 0x8022;
pub(super) const TPM_RH_OWNER: u32 = 0x4000_0001;

/// `sign` alone: the key verifies policy signatures and does nothing else.
pub(super) const P256_ATTRIBUTES: u32 = 0x0004_0000;

pub(super) const COORD_LEN: usize = 32;
pub(super) const DIGEST_LEN: usize = 32;

/// The PCRs the release approves: the kernel and its trailer.
pub(super) const APPROVED_PCRS: [u8; 1] = [9];
/// The PCRs this machine binds as they are: firmware, the bootloader as the
/// firmware measured it, and the Secure Boot state.
pub(super) const MACHINE_PCRS: [u8; 3] = [0, 4, 7];
