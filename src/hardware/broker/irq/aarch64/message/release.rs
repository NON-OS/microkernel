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

use crate::hardware::broker::irq::types::IrqError;
use crate::hardware::broker::pci_index;

use super::close::{close_all, close_vector};
use super::record::{MessageBind, BINDS};

/// `MkIrqUnmap` of one vector of a message bind: that vector alone, as on
/// x86. The bind goes with its last vector.
pub(in super::super) fn unmap_vector(pid: u32, grant_id: u64) -> Result<(), IrqError> {
    let mut binds = BINDS.lock();
    let holds = |b: &MessageBind| {
        let i = grant_id.wrapping_sub(b.base);
        b.pid == pid && b.base != 0 && i < b.count as u64 && b.live & 1 << i != 0
    };
    let slot = binds.iter_mut().find(|s| s.as_ref().is_some_and(holds));
    let slot = slot.ok_or(IrqError::UnknownGrant)?;
    if let Some(b) = slot.as_mut() {
        let handle = pci_index::lookup(b.device_id);
        close_vector(b, handle.as_ref(), (grant_id - b.base) as usize);
        if b.live == 0 {
            *slot = None;
        }
    }
    Ok(())
}

/// Close every bind `which` picks, whole. Returns the vectors closed.
pub(in super::super) fn release_where(which: impl Fn(&MessageBind) -> bool) -> usize {
    let mut binds = BINDS.lock();
    let mut closed = 0;
    for slot in binds.iter_mut() {
        if let Some(b) = slot.as_mut().filter(|b| which(b)) {
            closed += b.live.count_ones() as usize;
            close_all(b, pci_index::lookup(b.device_id).as_ref());
            *slot = None;
        }
    }
    closed
}
