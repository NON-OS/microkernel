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

//! A present in two halves: the body and the release counts it retires
//! against, then the compositor's answer. The chain moves only on that
//! answer, so a refused present leaves the shown buffer shown.

use super::super::fence;
use super::super::present::{Present, PRESENT_LEN};
use super::super::swapchain::MAX_BUFFERS;
use super::super::swapchain_damage::Damage;
use super::Swap;

/// A present built and not yet answered: send `body`, then `settle`.
pub struct Pending {
    pub body: [u8; PRESENT_LEN],
    index: usize,
    damage: Damage,
    counts: [u32; MAX_BUFFERS],
}

impl Swap {
    /// The present of buffer `i` with `damage`, in the buffer's pixels.
    pub fn present(&self, i: usize, damage: Damage) -> Pending {
        let index = u32::try_from(i).unwrap_or(u32::MAX);
        let handle = self.handles.get(i).copied().unwrap_or(0);
        let body = Present { handle, z: self.z, index, damage, opaque: self.opaque }.encode();
        // The count each buffer it retires waits past, read before the
        // compositor can see the present that retires it.
        let mut counts = [0u32; MAX_BUFFERS];
        for (j, c) in counts.iter_mut().enumerate().take(self.chain.count()) {
            *c = u32::try_from(fence::wait(self.sids[j], 0, 0)).unwrap_or(0);
        }
        Pending { body, index: i, damage, counts }
    }

    /// The compositor's answer to `p`: taken, the buffer is shown and the
    /// one before retires; refused, the buffer is free again.
    pub fn settle(&mut self, p: Pending, taken: bool) -> Result<(), &'static str> {
        let moved = match taken {
            true => self.chain.present(p.index, p.damage, |j| p.counts[j]),
            false => self.chain.cancel(p.index),
        };
        moved.map_err(|_| "buffer not taken to draw in")
    }

    /// Buffer `i`, taken to draw in, given back unpresented.
    pub fn cancel(&mut self, i: usize) -> bool {
        self.chain.cancel(i).is_ok()
    }
}
