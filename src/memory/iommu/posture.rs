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

use super::posture_line::{flag, identity_moved, posture_line};
use super::query::capabilities;
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

/// After a device moved between domains: print the line again if that
/// changed how many devices still reach all of memory.
pub(super) fn report_if_identity_moved() {
    if REPORTED.load(Ordering::Acquire) && identity_moved() {
        posture_line();
    }
}
