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

//! LE Enable Encryption as the peer's controller sees it.

use super::state::{Air, HANDLE};
use crate::hci_sim::events::status;
use crate::hci_sim::link_events::encryption_change;
use nonos_bt_wire::hci::opcode as op;
use nonos_bt_wire::l2cap::cid::SMP;

/// PIN or Key Missing (Vol 1, Part F, 2): the peer has another key.
const KEY_MISSING: u8 = 0x06;

impl Air {
    /// The link encrypts if the host's key is the one the responder
    /// derived; then the responder distributes its keys (Vol 3, Part H,
    /// 3.6.1). Any other key is refused as a peer with a different bond
    /// would refuse it.
    pub(super) fn encrypt(&mut self, p: &[u8]) -> Vec<Vec<u8>> {
        let mut ltk = [0u8; 16];
        for (k, b) in ltk.iter_mut().zip(p[12..28].iter().rev()) {
            *k = *b;
        }
        self.ltk = Some(ltk);
        let o = op::LE_ENABLE_ENCRYPTION.0;
        // A controller that will not start encryption says so at once.
        if let Some((_, why)) = self.cfg.fail.filter(|f| f.0 == o) {
            return vec![status(why, 1, o)];
        }
        if self.lies.forgot {
            return vec![status(0x02, 1, o)];
        }
        let start = status(0, 1, o);
        if self.refuse_encryption || ltk != self.smp.ltk {
            return vec![start, encryption_change(KEY_MISSING, HANDLE, 0)];
        }
        // Keys are distributed after pairing only; a link encrypted with a
        // kept key has nothing to distribute (Vol 3, Part H, 3.6.1).
        let keys = if self.seen.paired_here { self.smp.keys() } else { Vec::new() };
        for k in keys {
            self.send_to_host(SMP, &k);
        }
        self.lies.sealed = (true, true);
        vec![start, encryption_change(0, HANDLE, 1)]
    }
}
