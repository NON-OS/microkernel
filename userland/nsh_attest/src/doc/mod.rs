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

//! `NONOSATT` version 5, read exactly. Big-endian, every length bounded,
//! nothing left over. A version other than 5 is refused before any field
//! after it is read, so a version 3 or 4 document never reaches a check.

mod parse;

use crate::consts::{AK_LEN, PCR_COUNT, PCR_LEN};

/// A version 5 document, borrowed from the bytes it was read from.
///
/// `challenge`, `registry_root` and the DMA posture are authenticated only
/// through the quote's `extraData`; `capsule_count` and `registry_complete`
/// are not bound at all and are checked against the entries instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Doc<'a> {
    pub challenge: &'a [u8; 32],
    pub registry_root: &'a [u8; 32],
    pub capsule_count: u32,
    pub registry_complete: bool,
    pub iommu_vendor: u8,
    pub iommu_enforcing: bool,
    pub unconfined_grants: u32,
    /// Devices the IOMMU still lets reach all of memory.
    pub identity_devices: u32,
    /// The PCR values, in `QUOTED_PCRS_V4` order.
    pub pcrs: [&'a [u8; PCR_LEN]; PCR_COUNT],
    /// The firmware's TCG event log, as the loader carried it.
    pub log: &'a [u8],
    /// `TPMS_ATTEST`, byte for byte as the TPM signed it.
    pub attest: &'a [u8],
    /// `TPMT_SIGNATURE`, whole.
    pub signature: &'a [u8],
    pub ak_public: &'a [u8; AK_LEN],
}

impl<'a> Doc<'a> {
    /// The value of a quoted PCR, or `None` for one the document does not
    /// carry.
    pub fn pcr(&self, index: u8) -> Option<&'a [u8; PCR_LEN]> {
        let at = crate::consts::QUOTED_PCRS_V4.iter().position(|p| *p == index)?;
        self.pcrs.get(at).copied()
    }
}
