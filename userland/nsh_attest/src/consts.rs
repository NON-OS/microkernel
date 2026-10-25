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

//! The fixed shapes of a version 5 document and of what it carries.

/// The document's first eight bytes.
pub const DOC_MAGIC: &[u8; 8] = b"NONOSATT";
/// The only version this checker reads: K1's. Versions 3 and 4 quote PCRs
/// 0, 1, 2 and 7 alone, which names the firmware and not the NONOS chain;
/// 4 added the count of devices still reaching all of memory.
pub const DOC_VERSION: u32 = 5;
/// The largest document `HOST_PROOF` may carry.
pub const MAX_DOC: usize = 196_608;
/// The PCRs a version 5 quote selects, in the order the document carries
/// their values and the quote's digest covers them.
pub const QUOTED_PCRS_V4: [u8; 8] = [0, 1, 2, 4, 7, 8, 9, 14];
/// How many PCR values a version 5 document carries.
pub const PCR_COUNT: usize = QUOTED_PCRS_V4.len();
/// The PCRs the TCG event log must reproduce: the ones firmware extends.
/// PCRs 8, 9 and 14 are extended by NONOS code outside the firmware log and
/// are held to the release's values instead.
pub const REPLAYED_PCRS: [u8; 5] = [0, 1, 2, 4, 7];
/// The firmware PCRs the enrolment certificate fixes, in its order.
pub const ENROLLED_PCRS: [u8; 4] = [0, 1, 2, 7];
/// The largest TCG event log a document carries.
pub const MAX_LOG: usize = 131_072;
/// The largest `TPMS_ATTEST`. A quote over one bank is about 145 bytes.
pub const MAX_ATTEST: usize = 1024;
/// The largest ECDSA P-256 `TPMT_SIGNATURE`: two algorithms, then r and s
/// of at most 32 bytes each, each after its size.
pub const MAX_SIG: usize = 2 + 2 + 2 + 32 + 2 + 32;
/// The attestation key's uncompressed point, x then y.
pub const AK_LEN: usize = 64;
/// One SHA-256 PCR value.
pub const PCR_LEN: usize = 32;

/// `iommu_vendor` values the kernel writes; any other is refused.
pub const IOMMU_NONE: u8 = 0;
pub const IOMMU_ARM_SMMU_V3: u8 = 3;
