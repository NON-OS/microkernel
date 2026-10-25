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

//! The three claims the document settles on its own.
//!
//! The challenge echo is the anti-replay check and the only part of the document
//! this capsule can verify without the key; the completeness flag is the machine
//! saying whether it still knows everything it is running; the DMA row says
//! whether device DMA is held by the IOMMU, which needs the unit enforcing and
//! no mapping going around it, not the first alone. Its evidence names the unit
//! the kernel reported, so "no IOMMU" reads differently from one bypassed.

use crate::about::data::doc_parse::Doc;
use crate::about::format::u64_decimal;

/// "VT-d, 0 unconfined", or "no IOMMU" when there is no unit to confine
/// anything; "VT-d, 3 open" when devices it still lets reach all of memory
/// make the verdict fail. The longest is 29 bytes.
pub(super) fn dma_evidence(out: &mut [u8; 40], doc: &Doc) -> usize {
    let Some(unit) = doc.iommu_unit() else {
        return put(out, 0, b"no IOMMU");
    };
    let mut digits = [0u8; 20];
    let mut n = put(out, 0, unit);
    n = put(out, n, b", ");
    // The count that fails the verdict, unconfined grants first.
    let (count, what): (u32, &[u8]) = match (doc.unconfined_grants, doc.identity_devices) {
        (0, open) if open > 0 => (open, b" open"),
        (grants, _) => (grants, b" unconfined"),
    };
    n = put(out, n, u64_decimal(u64::from(count), &mut digits));
    put(out, n, what)
}

fn put(out: &mut [u8; 40], at: usize, s: &[u8]) -> usize {
    let n = s.len().min(out.len() - at);
    out[at..at + n].copy_from_slice(&s[..n]);
    at + n
}
