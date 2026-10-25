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

//! The DMA row's evidence: the unit the kernel reported and how many mappings
//! go around it, or "no IOMMU" when there is none.

use crate::about::data::doc_parse::Doc;
use crate::about::format::u64_decimal;

/// "VT-d, 0 unconfined", or "no IOMMU" when there is no unit to confine
/// anything. The longest, "AMD-Vi, " with ten digits and " unconfined", is 29.
pub(super) fn dma_evidence(out: &mut [u8; 40], doc: &Doc) -> usize {
    let Some(unit) = doc.iommu_unit() else {
        return put(out, 0, b"no IOMMU");
    };
    let mut digits = [0u8; 20];
    let mut n = put(out, 0, unit);
    n = put(out, n, b", ");
    n = put(out, n, u64_decimal(u64::from(doc.unconfined_grants), &mut digits));
    put(out, n, b" unconfined")
}

fn put(out: &mut [u8; 40], at: usize, s: &[u8]) -> usize {
    let n = s.len().min(out.len() - at);
    out[at..at + n].copy_from_slice(&s[..n]);
    at + n
}
