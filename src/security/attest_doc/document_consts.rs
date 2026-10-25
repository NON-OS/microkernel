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

//! The document's fixed numbers: its magic and version, the IOMMU values it
//! may carry, the PCRs it quotes and the most log it holds.

/// Wire format version. A verifier that does not recognise it must refuse
/// rather than parse optimistically: a document it half understands is worse
/// than one it rejects. Version 5 (K1) quotes the PCRs NONOS extends and
/// carries their values and the firmware's event log.
pub const DOC_VERSION: u32 = 5;

pub const DOC_MAGIC: &[u8; 8] = b"NONOSATT";

/// `iommu_vendor` values. A verifier that meets another value must refuse.
pub const IOMMU_NONE: u8 = 0;
pub const IOMMU_INTEL_VTD: u8 = 1;
pub const IOMMU_AMD_VI: u8 = 2;
pub const IOMMU_ARM_SMMU_V3: u8 = 3;

/// The PCRs quoted, in the order the document carries their values and the
/// quote's digest covers them: the firmware's (0, 1, 2, 7), the boot
/// manager and loader's Authenticode (4), the loader (8), the admitted
/// kernel (9) and the capsule and proof measurements (14).
pub const QUOTED_PCRS: [u8; 8] = [0, 1, 2, 4, 7, 8, 9, 14];

/// The largest event log a document carries: the loader's own cap
/// (`nonos-boot-measure`, `MAX_LOG_BYTES`), so every log a machine boots
/// with can be attested. A longer one is refused, not cut, since a verifier
/// replays it whole.
pub const MAX_LOG: usize = 1024 * 1024;

/// Whether a log fits a document.
pub(crate) const fn log_fits(log: &[u8]) -> bool {
    log.len() <= MAX_LOG
}
