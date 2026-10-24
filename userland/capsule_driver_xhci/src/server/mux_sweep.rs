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

//! Giving back what clients that exited held, on every controller
//! (`sweep_orphans`), and the multiplexer's slot ids that named it or that
//! such a client had only reserved. A shared device a live driver still
//! claims an interface of, and a child parked below a hub, stay.

use nonos_libc::mk_pid_alive;

use super::mux::Mux;
use super::mux_forward::VSlot;
use super::sweep::sweep_orphans;
use nonos_usb_virt::Virt;

impl Mux {
    pub fn sweep(&mut self) {
        let single = self.ctxs.len() == 1;
        for (i, ctx) in self.ctxs.iter_mut().enumerate() {
            let (vslots, share, virt): (_, _, &Virt) = (&self.vslots, &self.share, &self.virt);
            // The multiplexer's id for this controller's `slot`.
            let id = |slot: u8| match single {
                true => Some(slot),
                false => (1..=255u8)
                    .find(|&v| vslots[v as usize] == VSlot::Bound { ctrl: i as u8, slot }),
            };
            let keep = |slot: u8| {
                id(slot).is_some_and(|v| {
                    share.claims.held_live(v, |p| mk_pid_alive(p)) || virt.knows(v)
                })
            };
            sweep_orphans(ctx, keep);
        }
        // What was kept about an id whose slot is gone goes too.
        for v in 1..=255u8 {
            let gone = self.at(v).is_some_and(|(c, l)| {
                let d = &self.ctxs[c].driver;
                !d.slots.is_allocated(l, d.layout.max_slots)
            });
            if gone {
                self.share.forget(v);
            }
        }
        for v in self.vslots.iter_mut() {
            let gone = match *v {
                VSlot::Pending { owner } => owner != 0 && !mk_pid_alive(owner),
                VSlot::Bound { ctrl, slot } => self
                    .ctxs
                    .get(ctrl as usize)
                    .is_some_and(|c| !c.driver.slots.is_allocated(slot, c.driver.layout.max_slots)),
                VSlot::Free => false,
            };
            if gone {
                *v = VSlot::Free;
            }
        }
    }
}
