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

//! Where each field sits.

// Big-endian throughout, matching the TPM structures the document carries.
pub(super) const MAGIC: &[u8; 8] = b"NONOSATT";
pub(super) const VERSION: u32 = 3;

// A TPMS_ATTEST with an ECDSA signature lands well inside this. The syscall
// refuses rather than truncating if it does not fit, so a short read here is
// reported as a refusal rather than parsed as a short document.
pub const DOC_CAP: usize = 2048;

pub(super) const ROOT_AT: usize = 8 + 4 + 32;
pub(super) const COUNT_AT: usize = ROOT_AT + 32;
pub(super) const COMPLETE_AT: usize = COUNT_AT + 4;
pub(super) const VENDOR_AT: usize = COMPLETE_AT + 1;
pub(super) const ENFORCING_AT: usize = VENDOR_AT + 1;
pub(super) const UNCONFINED_AT: usize = ENFORCING_AT + 1;
pub(super) const ATTEST_LEN_AT: usize = UNCONFINED_AT + 4;

/// Highest `iommu_vendor` value the kernel writes: none, VT-d, AMD-Vi.
pub(super) const VENDOR_MAX: u8 = 2;

/// The attestation key closes the document: an uncompressed P-256 point, x then y.
pub(super) const KEY_LEN: u32 = 64;
