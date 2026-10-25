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

//! The posture line itself, the first of the two `report_posture` prints and
//! the one printed again whenever a count it carries changes.
//!
//! `identity devices` is the third fact beside the two: devices the kernel
//! found and no capsule has claimed, which the unit still lets reach all of
//! memory (VT-d's boot identity domain, AMD-Vi's passthrough entries). Grants
//! can all be confined while these are not, so the line never leaves it out.
//! A development opt-in that lets DMA or interrupts past the unit is said at
//! its end (dev_unconfined, dev_unremapped).

use core::sync::atomic::{AtomicUsize, Ordering};

use super::backend::identity_devices;
use super::query::capabilities;
use super::unconfined::unconfined_grants;
use crate::sys::serial::Line;

/// The identity count the last line printed, so a claim that leaves it as it
/// was prints nothing.
static PRINTED_IDENTITY: AtomicUsize = AtomicUsize::new(usize::MAX);

/*
 * The vendor is "none" on a machine with neither DMAR remapping units nor an
 * IVRS table, so every line has the same shape for the boot matrix to read.
 */
pub(super) fn posture_line() {
    let caps = capabilities();
    let identity = identity_devices();
    PRINTED_IDENTITY.store(identity, Ordering::Release);
    Line::new()
        .str(b"[IOMMU] ")
        .str(caps.vendor.name())
        .str(b" present, enforcing=")
        .str(flag(caps.enforcing))
        .str(b", unconfined grants=")
        .dec(u64::from(unconfined_grants()))
        .str(b", identity devices=")
        .dec(identity as u64)
        .str(super::dev_unconfined::posture_tail())
        .str(super::dev_unremapped::posture_tail())
        .str(super::no_iommu::posture_tail())
        .end();
}

/// Whether the identity count moved since the last line.
pub(super) fn identity_moved() -> bool {
    identity_devices() != PRINTED_IDENTITY.load(Ordering::Acquire)
}

pub(super) const fn flag(on: bool) -> &'static [u8] {
    if on {
        b"1"
    } else {
        b"0"
    }
}
