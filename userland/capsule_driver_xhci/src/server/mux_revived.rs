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

//! What a controller brought back since its slots were bound leaves: it
//! dropped every slot it had, so the ids bound to it name nothing now and
//! are free, and what was kept about them (claims, pipes, holders,
//! standings) goes with them. Parked children go too: their hub was a
//! device of a controller that dropped it, or may have been, and
//! driver.usb_hub0 parks them again once it has walked the hub anew.

use super::mux::{Mux, VSlot};
use nonos_usb_virt::Virt;

impl Mux {
    pub(super) fn forget_revived(&mut self) {
        let single = self.ctxs.len() == 1;
        for (i, ctx) in self.ctxs.iter().enumerate() {
            if self.generations[i] == ctx.driver.generation {
                continue;
            }
            self.generations[i] = ctx.driver.generation;
            self.virt = Virt::default();
            for (v, slot) in self.vslots.iter_mut().enumerate() {
                let bound = matches!(*slot, VSlot::Bound { ctrl, .. } if ctrl as usize == i);
                if bound {
                    *slot = VSlot::Free;
                }
                // With one controller the ids are its own, and none is bound.
                if bound || single {
                    self.share.forget(v as u8);
                }
            }
        }
    }
}
