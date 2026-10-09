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

use nonos_app_skeleton::PaintBuffer;

use crate::about::data::doc_parse::Doc;
use crate::about::data::verify::Verdict;
use crate::about::format::u64_decimal;

use super::super::kv::ROW_H;
use super::super::metrics::CARD_PAD;
use super::verify_row::row;

pub(super) fn rows(fb: &mut PaintBuffer, rows_y: i32, inner: u32, doc: &Doc) {
    row(
        fb,
        CARD_PAD,
        rows_y,
        inner,
        Verdict::from_bool(doc.challenge_echoed),
        b"challenge came back unchanged",
        b"",
    );
    row(
        fb,
        CARD_PAD,
        rows_y + ROW_H as i32,
        inner,
        Verdict::from_bool(doc.registry_complete),
        b"machine recorded everything it runs",
        b"",
    );
    let mut ev = [0u8; 40];
    let n = dma_evidence(&mut ev, doc);
    row(
        fb,
        CARD_PAD,
        rows_y + (ROW_H * 2) as i32,
        inner,
        Verdict::from_bool(doc.iommu_enforcing && doc.unconfined_grants == 0),
        b"device DMA held by the IOMMU",
        &ev[..n],
    );
}

/// "VT-d, 0 unconfined", or "no IOMMU" when there is no unit to confine
/// anything. The longest, "AMD-Vi, " with ten digits and " unconfined", is 29.
fn dma_evidence(out: &mut [u8; 40], doc: &Doc) -> usize {
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
