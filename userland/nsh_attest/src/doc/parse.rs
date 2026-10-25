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

//! The read itself, field by field in the order the document lays them out.

use super::Doc;
use crate::consts::{
    AK_LEN, DOC_MAGIC, DOC_VERSION, IOMMU_ARM_SMMU_V3, MAX_ATTEST, MAX_DOC, MAX_LOG, MAX_SIG,
    PCR_COUNT, QUOTED_PCRS_V4,
};
use crate::read::Be;
use crate::{FormatFault, Refusal};

impl<'a> Doc<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Refusal> {
        if bytes.len() > MAX_DOC {
            return Err(Refusal::Format(FormatFault::Oversize));
        }
        let mut r = Be::new(bytes);
        if r.array::<8>().map_err(Refusal::Format)? != DOC_MAGIC {
            return Err(Refusal::Format(FormatFault::Magic));
        }
        match r.u32().map_err(Refusal::Format)? {
            DOC_VERSION => body(r).map_err(Refusal::Format),
            other => Err(Refusal::Version(other)),
        }
    }
}

fn body(mut r: Be<'_>) -> Result<Doc<'_>, FormatFault> {
    let challenge = r.array()?;
    let registry_root = r.array()?;
    let capsule_count = r.u32()?;
    let registry_complete = r.flag()?;
    let iommu_vendor = r.u8()?;
    if iommu_vendor > IOMMU_ARM_SMMU_V3 {
        return Err(FormatFault::Value);
    }
    let iommu_enforcing = r.flag()?;
    let unconfined_grants = r.u32()?;
    let identity_devices = r.u32()?;
    if r.u8()? as usize != PCR_COUNT {
        return Err(FormatFault::Value);
    }
    let mut pcrs = [&[0u8; 32]; PCR_COUNT];
    for (slot, want) in pcrs.iter_mut().zip(QUOTED_PCRS_V4) {
        if r.u8()? != want {
            return Err(FormatFault::Value);
        }
        *slot = r.array()?;
    }
    let log = r.sized(true, 0, MAX_LOG)?;
    let attest = r.sized(true, 1, MAX_ATTEST)?;
    let signature = r.sized(true, 1, MAX_SIG)?;
    let ak_public = r.sized(true, AK_LEN, AK_LEN)?.try_into().map_err(|_| FormatFault::Value)?;
    r.finish()?;
    #[rustfmt::skip]
    let doc = Doc { challenge, registry_root, capsule_count, registry_complete, iommu_vendor,
        iommu_enforcing, unconfined_grants, identity_devices, pcrs, log, attest, signature,
        ak_public };
    Ok(doc)
}
