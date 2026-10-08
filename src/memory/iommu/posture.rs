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

//! The one line that says what the IOMMU is doing on this boot.
//!
//! Two facts, never one without the other: whether the unit is enforcing, and
//! how many mappings devices can reach with no domain confining them. A unit in
//! service with unconfined grants beside it is a mechanism present and a
//! property absent, and the line says both. Only when the count is zero does
//! `enforcing=1` mean that device DMA is confined.

use core::sync::atomic::{AtomicBool, Ordering};

use super::query::capabilities;
use super::unconfined::unconfined_grants;
use crate::sys::serial::Line;

static REPORTED: AtomicBool = AtomicBool::new(false);

/*
 * Built from the capability query and the unconfined count and nothing else,
 * so the line and the query cannot disagree. The boot matrix reads the vendor,
 * enforcing= and the count from the last posture line in a log.
 */
pub fn report_posture() {
    let caps = capabilities();
    posture_line();
    Line::new()
        .str(b"[IOMMU] capabilities aw=")
        .dec(u64::from(caps.address_width_bits))
        .str(b" ir=")
        .str(flag(caps.interrupt_remapping))
        .str(b" snoop=")
        .str(flag(caps.snoop_control))
        .str(b" pages=")
        .hex(caps.page_sizes)
        .str(b" domains=")
        .dec(u64::from(caps.domain_count))
        .str(b" faults=")
        .dec(crate::arch::x86_64::iommu::unit::fault::fault_total())
        .end();
    REPORTED.store(true, Ordering::Release);
}

/// Print the posture line again after the unconfined count changed, once the
/// boot has reported it the first time. Before that nothing has been said, and
/// on a build that never reports a posture nothing is said at all.
pub(super) fn report_again() {
    if REPORTED.load(Ordering::Acquire) {
        posture_line();
    }
}

/*
 * The vendor is "none" on a machine with neither DMAR remapping units nor an
 * IVRS table, so every line has the same shape for the boot matrix to read.
 */
fn posture_line() {
    let caps = capabilities();
    Line::new()
        .str(b"[IOMMU] ")
        .str(caps.vendor.name())
        .str(b" present, enforcing=")
        .str(flag(caps.enforcing))
        .str(b", unconfined grants=")
        .dec(u64::from(unconfined_grants()))
        .end();
}

const fn flag(on: bool) -> &'static [u8] {
    if on {
        b"1"
    } else {
        b"0"
    }
}
