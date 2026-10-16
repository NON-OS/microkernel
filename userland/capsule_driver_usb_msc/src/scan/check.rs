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

//! A bound device that failed a request is asked, in one step, whether it
//! is still on its port, ready, and of the size it was. One gone or not
//! ready is let go, and only the same device is served again (`identity`);
//! a card taken out ends serving (`release`). A medium change, as a card
//! swapped in a reader says (UNIT ATTENTION 28h) or a new size shows, ends
//! serving once the kernel has read or written a sector: it chose its disk
//! by what it read, and two cards of one size cannot be told apart. Any
//! other UNIT ATTENTION, a sense that did not read, or a command lost on
//! the bus may be a change too (`Said`): the medium is served afresh, which
//! raises the generation the kernel reads before it writes. One was kept
//! with no sign, and a card swapped in under a reset's 29h, or one whose
//! 28h went with a lost CSW, took the kernel's next write. Until the
//! kernel holds a sector, a medium said to have changed is served afresh at
//! its size, and the request asked again (server/handlers/block.rs): a
//! reader that said so on its first READ lost the boot disk.
//! A device is kept after three failed requests in a row at most (`keep`).

use super::ready_size::ready_size;
use super::scanner::Scanner;
use crate::disk::Disk;
use crate::protocol::E_NODEV;
use crate::state::{Said, State};
use crate::xhci::connected_ports;

impl Scanner {
    pub(super) fn check(&mut self, xhci: u32, disk: Disk, state: &mut State) {
        let (held, before) = (state.held(), state.take_said());
        // Said under the request that failed: the new card is asked nothing.
        if held && before == Said::Changed {
            return self.end(xhci, disk, E_NODEV);
        }
        // A listing that failed says nothing of the device; it is asked.
        let gone = connected_ports(xhci).is_ok_and(|on| !on.iter().any(|p| p.id == disk.port));
        let size = if gone { Err(E_NODEV) } else { ready_size(&disk, state) };
        let same = size == Ok((disk.blocks, disk.block_len));
        match (size, before.max(state.take_said()), self.served) {
            (_, Said::Changed, _) if held => self.end(xhci, disk, E_NODEV),
            (Ok((blocks, block_len)), said, Some(id))
                if said > Said::Nothing && (same || !held) =>
            {
                let disk = Disk { blocks, block_len, ..disk };
                self.keep(xhci, disk, Some(id.sized(&disk)), state);
            }
            (Ok(_), Said::Nothing, _) if same => self.keep(xhci, disk, None, state),
            (Ok(_), _, _) => self.end(xhci, disk, E_NODEV),
            (Err(e), _, _) => self.let_go(xhci, disk, e),
        }
    }
}
