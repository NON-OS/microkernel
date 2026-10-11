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

//! Giving back what a bind took, a vector at a time and in the reverse of
//! the order it was taken: the function stops writing, the controller drops
//! the vector and anything it left pending, then its handler and slot go.

use crate::arch::aarch64::gic::msi;
use crate::hardware::broker::pci_index::PciHandle;

use super::record::MessageBind;
use super::{entries, silence};

const PAGE: u64 = 0x1000;

/// Close vector `i`. A controller that does not confirm dropping it keeps
/// the vector out of use, handler and all, where the trampoline finds no
/// slot for it and counts nothing.
pub(super) fn close_vector(b: &mut MessageBind, handle: Option<&PciHandle>, i: usize) {
    if i >= b.count || b.live & 1 << i == 0 {
        return;
    }
    let v = b.vectors[i];
    silence::vector(b, handle, i);
    let dropped = msi::unbind(b.msi_id, &[v]).is_ok();
    if !dropped {
        crate::log::info!(
            "[IRQ] intid {} kept out of use: the MSI controller did not drop it",
            v.intid
        );
    }
    if b.base != 0 {
        entries::close(b.base + i as u64, dropped.then_some(v.intid));
    }
    b.live &= !(1 << i);
    if b.live == 0 {
        last(b, handle);
    }
}

/// Close every vector, then whatever a bind that failed early still holds.
pub(super) fn close_all(b: &mut MessageBind, handle: Option<&PciHandle>) {
    (0..b.count).for_each(|i| close_vector(b, handle, i));
    last(b, handle);
}

/// With no vector left the function's MSI-X goes off, and the doorbell
/// leaves the capsule's domain.
fn last(b: &mut MessageBind, handle: Option<&PciHandle>) {
    silence::function(b, handle);
    if b.doorbell != 0 {
        if !crate::hardware::broker::confine::unmap(b.pid, b.doorbell, PAGE, b.confined) {
            crate::log::info!("[IRQ] doorbell iova {:#x} left mapped", b.doorbell);
        }
        b.doorbell = 0;
    }
}
