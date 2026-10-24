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

//! What the device being asked came to: bound, or given up. One not taken
//! gives its slot back, and its port waits, from the first time when it had
//! no medium. Once a device has been served only the same unit is served
//! again (`sized`); another is given back, and its port is not asked while
//! it stays. The unit served found changed ends serving (`release`).

use super::ask::Step;
use super::identity::Identity;
use super::medium::Medium;
use super::report::{Report, Stage};
use super::scanner::Scanner;
use crate::disk::Disk;
use crate::protocol::E_NOMEDIUM;
use crate::xhci::disable_slot;

impl Scanner {
    pub(super) fn asked(&mut self, xhci: u32, step: Step) {
        let (connected, closed) = (self.report.connected, self.report.closed);
        let (disk, stage, e) = match step {
            Step::Again(asking) => {
                self.asking = Some(asking);
                return;
            }
            Step::Ours(disk, id) => return self.serve(disk, id),
            Step::Failed(disk, Stage::Changed, e) => return self.end(xhci, disk, e),
            Step::Failed(disk, stage, e) => (disk, stage, e),
        };
        disable_slot(xhci, disk.slot);
        match stage {
            Stage::Other => self.ports_of(xhci, |p| p.set(disk.port, true)),
            _ => self.ports_of(xhci, |p| p.failed(disk.port, e == E_NOMEDIUM)),
        }
        self.report = Report { connected, closed, ..Report::at(stage, disk.port, e) };
    }

    /// Serve `disk`, named `id`, from now on. A device served after the
    /// first, or a medium taken afresh (`check`), raises the re-bind
    /// generation the kernel reads (server/handlers/get_state.rs), so it can
    /// read what it trusts of the disk again before it writes. Its port
    /// keeps the failures it had, so a device let go time after time backs
    /// off.
    pub(super) fn serve(&mut self, disk: Disk, id: Identity) {
        if self.served.replace(id).is_some() {
            self.rebinds = self.rebinds.wrapping_add(1);
        }
        let (aux, blocks, connected) = (disk.block_len, disk.blocks, self.report.connected);
        self.report = Report { aux, blocks, connected, ..Report::at(Stage::Bound, disk.port, 0) };
        (self.medium, self.searching) = (Medium::Ready(disk), false);
    }
}
