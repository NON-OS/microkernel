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

//! Step 3 for a version 5 document: the quote over `QUOTED_PCRS_V4`, its
//! `extraData` recomputed by the client from the document's challenge,
//! registry root and DMA posture, its digest over the document's values.

use super::binding::{qualifying_data, DmaPosture};
use crate::consts::QUOTED_PCRS_V4;
use crate::quote::check_quote;
use crate::{Doc, Refusal};

pub fn quote(doc: &Doc<'_>) -> Result<(), Refusal> {
    let dma = DmaPosture {
        vendor: doc.iommu_vendor,
        enforcing: doc.iommu_enforcing,
        unconfined_grants: doc.unconfined_grants,
        identity_devices: doc.identity_devices,
    };
    let extra = qualifying_data(doc.challenge, doc.registry_root, &dma);
    check_quote(doc.attest, doc.signature, doc.ak_public, &extra, &QUOTED_PCRS_V4, &doc.pcrs)?;
    Ok(())
}
