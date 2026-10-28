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
//! Who may name a slot: the client that enabled it, as driver.xhci0's
//! multiplexer holds its slots (capsule_driver_xhci/src/slots/holders.rs);
//! anyone else is answered E_PERM. A holder the kernel says is gone is
//! replaced by the next client that asks, so a restarted driver can give
//! back what its last life left, and the sweep gives back the slots of
//! the gone that no one asked for, but a child parked below a hub.

use super::front::{Bind, Front};
use super::give_back::disable;
use super::hc::Hc;
use crate::wire::ops::OP_DISABLE_SLOT;
use crate::wire::Request;

impl<H: Hc> Front<H> {
    /// Whether `pid` may act on `slot`. A slot not given out has no
    /// holder, and the request is refused further on as invalid.
    pub fn admits(&mut self, slot: u8, pid: u32) -> bool {
        let at = (slot as usize).wrapping_sub(1);
        match (self.bound.get(at), self.owner.get(at).copied()) {
            (None | Some(Bind::Free), _) | (_, None) => true,
            (_, Some(h)) if h == pid => true,
            (_, Some(h)) if !(self.alive)(h) => {
                self.owner[at] = pid;
                true
            }
            _ => false,
        }
    }

    /// Every slot whose holder is gone given back, as its holder would
    /// have: a claim let go, a slot disabled on its controller. `tx` takes
    /// the replies no one reads.
    pub fn sweep(&mut self, tx: &mut [u8]) {
        let req = Request { op: OP_DISABLE_SLOT, flags: 0, request_id: 0, payload_len: 1 };
        for slot in 1..=self.bound.len() as u8 {
            let at = slot as usize - 1;
            let parked = self.virt.knows(slot) && self.virt.real(slot) == slot;
            let (held, h) = (self.bound[at] != Bind::Free, self.owner[at]);
            if held && !parked && !(self.alive)(h) {
                disable(self, &req, &[slot], tx);
            }
        }
    }
}
