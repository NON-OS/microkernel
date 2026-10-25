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

//! Faulting devices, noted where a fault is read and blocked where locks may
//! be taken (K12). The fault poll runs in the timer interrupt, where the
//! tables' and queues' locks could be held by the code it interrupted, so
//! it only notes the requester id here, without a lock; the broker blocks
//! it on its next IRQ wait, which every driver makes again and again.

use super::noted::Noted;
use super::route::{route, Backend};

/// A fault past a full table is noted again on its next fault.
static NOTED: Noted<16> = Noted::new();

pub(crate) fn note_fault(source: u16) {
    let _ = NOTED.note(source);
}

/// Block every noted device. Called where the locks may be taken.
pub(crate) fn block_faulted() {
    for source in NOTED.take() {
        let blocked = match route(b"block_faulted") {
            Ok(Backend::Vtd) => crate::arch::x86_64::iommu::device::block_device(source).ok(),
            #[cfg(feature = "nonos-iommu-amdvi")]
            Ok(Backend::AmdVi) => crate::arch::x86_64::iommu::amd_vi::domain::block(source).ok(),
            Err(_) => None,
        };
        if blocked == Some(true) {
            crate::sys::serial::println(b"[IOMMU] a faulting device is blocked until released");
        }
    }
}
