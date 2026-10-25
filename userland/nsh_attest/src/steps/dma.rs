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

//! Step 6: device DMA is confined. An IOMMU is present and enforcing, no
//! mapping goes around it, and no device is left in a domain that reaches
//! all of memory. All four values are bound into the quote.

use crate::consts::IOMMU_NONE;
use crate::{Doc, Refusal};

pub fn dma(doc: &Doc<'_>) -> Result<(), Refusal> {
    let present = doc.iommu_vendor != IOMMU_NONE && doc.iommu_enforcing;
    match present && doc.unconfined_grants == 0 && doc.identity_devices == 0 {
        true => Ok(()),
        false => Err(Refusal::Dma),
    }
}
